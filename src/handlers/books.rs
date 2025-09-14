use rocket::{get, post, put, delete, http::Status, serde::json::Json};
use uuid::Uuid;
use crate::{
    models::{ApiResponse},
    models::book::{
        BookWithCategory, Category, CreateBookRequest, UpdateBookRequest, 
        CreateCategoryRequest
    },
    database::get_pool,
};

// Listar todos os livros públicos
#[get("/books")]
pub async fn get_books() -> Result<Json<ApiResponse<Vec<BookWithCategory>>>, Status> {
    let pool = get_pool();
    
    match sqlx::query_as::<_, BookWithCategory>(
        r#"
        SELECT b.*, c.id as cat_id, c.name as cat_name, c.description as cat_description, 
               c.created_at as cat_created_at, c.updated_at as cat_updated_at
        FROM books b
        JOIN categories c ON b.category_id = c.id
        WHERE b.is_public = true
        ORDER BY b.created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
    {
        Ok(books) => Ok(Json(ApiResponse::success(books, "Livros listados com sucesso"))),
        Err(e) => {
            eprintln!("Erro ao buscar livros: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

// Buscar livro por ID
#[get("/books/<id>")]
pub async fn get_book(id: String) -> Result<Json<ApiResponse<BookWithCategory>>, Status> {
    let pool = get_pool();
    let book_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return Err(Status::BadRequest),
    };

    match sqlx::query_as::<_, BookWithCategory>(
        r#"
        SELECT b.*, c.id as cat_id, c.name as cat_name, c.description as cat_description, 
               c.created_at as cat_created_at, c.updated_at as cat_updated_at
        FROM books b
        JOIN categories c ON b.category_id = c.id
        WHERE b.id = $1 AND b.is_public = true
        "#
    )
    .bind(book_id)
    .fetch_one(pool)
    .await
    {
        Ok(book) => Ok(Json(ApiResponse::success(book, "Livro encontrado"))),
        Err(sqlx::Error::RowNotFound) => Err(Status::NotFound),
        Err(e) => {
            eprintln!("Erro ao buscar livro: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

// Criar novo livro
#[post("/books", data = "<book_data>")]
pub async fn create_book(book_data: Json<CreateBookRequest>) -> Result<Json<ApiResponse<BookWithCategory>>, Status> {
    let pool = get_pool();
    let book = &book_data.into_inner();

    // TODO: Implementar autenticação para obter user_id do token
    // Por enquanto, vamos usar um user_id fixo para demonstração
    let user_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();

    match sqlx::query_as::<_, BookWithCategory>(
        r#"
        WITH new_book AS (
            INSERT INTO books (title, author, isbn, description, content, category_id, user_id, is_public)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
        )
        SELECT b.*, c.id as cat_id, c.name as cat_name, c.description as cat_description, 
               c.created_at as cat_created_at, c.updated_at as cat_updated_at
        FROM new_book b
        JOIN categories c ON b.category_id = c.id
        "#
    )
    .bind(&book.title)
    .bind(&book.author)
    .bind(&book.isbn)
    .bind(&book.description)
    .bind(&book.content)
    .bind(&book.category_id)
    .bind(user_id)
    .bind(book.is_public)
    .fetch_one(pool)
    .await
    {
        Ok(new_book) => Ok(Json(ApiResponse::success(new_book, "Livro criado com sucesso"))),
        Err(sqlx::Error::Database(db_err)) if db_err.constraint().is_some() => {
            Err(Status::Conflict) // Categoria não existe
        }
        Err(e) => {
            eprintln!("Erro ao criar livro: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

// Atualizar livro
#[put("/books/<id>", data = "<book_data>")]
pub async fn update_book(id: String, book_data: Json<UpdateBookRequest>) -> Result<Json<ApiResponse<BookWithCategory>>, Status> {
    let pool = get_pool();
    let book_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return Err(Status::BadRequest),
    };

    let book = &book_data.into_inner();

    // Construir query dinamicamente
    let mut query = r#"
        WITH updated_book AS (
            UPDATE books SET 
    "#.to_string();
    
    let mut params: Vec<String> = Vec::new();
    let mut param_count = 1;

    if let Some(ref _title) = book.title {
        params.push(format!("title = ${}", param_count));
        param_count += 1;
    }
    if let Some(ref _author) = book.author {
        params.push(format!("author = ${}", param_count));
        param_count += 1;
    }
    if let Some(ref _isbn) = book.isbn {
        params.push(format!("isbn = ${}", param_count));
        param_count += 1;
    }
    if let Some(ref _description) = book.description {
        params.push(format!("description = ${}", param_count));
        param_count += 1;
    }
    if let Some(ref _content) = book.content {
        params.push(format!("content = ${}", param_count));
        param_count += 1;
    }
    if let Some(_category_id) = book.category_id {
        params.push(format!("category_id = ${}", param_count));
        param_count += 1;
    }
    if let Some(_is_public) = book.is_public {
        params.push(format!("is_public = ${}", param_count));
        param_count += 1;
    }

    if params.is_empty() {
        return Err(Status::BadRequest);
    }

    params.push(format!("updated_at = NOW()"));
    query.push_str(&params.join(", "));
    query.push_str(&format!(" WHERE id = ${} RETURNING *", param_count));
    query.push_str(r#"
        )
        SELECT b.*, c.id as cat_id, c.name as cat_name, c.description as cat_description, 
               c.created_at as cat_created_at, c.updated_at as cat_updated_at
        FROM updated_book b
        JOIN categories c ON b.category_id = c.id
    "#);

    let mut query_builder = sqlx::query_as::<_, BookWithCategory>(&query);
    
    if let Some(ref _title) = book.title {
        query_builder = query_builder.bind(_title);
    }
    if let Some(ref _author) = book.author {
        query_builder = query_builder.bind(_author);
    }
    if let Some(ref _isbn) = book.isbn {
        query_builder = query_builder.bind(_isbn);
    }
    if let Some(ref _description) = book.description {
        query_builder = query_builder.bind(_description);
    }
    if let Some(ref _content) = book.content {
        query_builder = query_builder.bind(_content);
    }
    if let Some(_category_id) = book.category_id {
        query_builder = query_builder.bind(_category_id);
    }
    if let Some(_is_public) = book.is_public {
        query_builder = query_builder.bind(_is_public);
    }
    query_builder = query_builder.bind(book_id);

    match query_builder.fetch_one(pool).await {
        Ok(updated_book) => Ok(Json(ApiResponse::success(updated_book, "Livro atualizado com sucesso"))),
        Err(sqlx::Error::RowNotFound) => Err(Status::NotFound),
        Err(sqlx::Error::Database(db_err)) if db_err.constraint().is_some() => {
            Err(Status::Conflict) // Categoria não existe
        }
        Err(e) => {
            eprintln!("Erro ao atualizar livro: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

// Deletar livro
#[delete("/books/<id>")]
pub async fn delete_book(id: String) -> Result<Json<ApiResponse<()>>, Status> {
    let pool = get_pool();
    let book_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return Err(Status::BadRequest),
    };

    match sqlx::query("DELETE FROM books WHERE id = $1")
        .bind(book_id)
        .execute(pool)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => {
            Ok(Json(ApiResponse::success((), "Livro deletado com sucesso")))
        }
        Ok(_) => Err(Status::NotFound),
        Err(e) => {
            eprintln!("Erro ao deletar livro: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

// Listar categorias
#[get("/categories")]
pub async fn get_categories() -> Result<Json<ApiResponse<Vec<Category>>>, Status> {
    let pool = get_pool();
    
    match sqlx::query_as::<_, Category>("SELECT * FROM categories ORDER BY name")
        .fetch_all(pool)
        .await
    {
        Ok(categories) => Ok(Json(ApiResponse::success(categories, "Categorias listadas com sucesso"))),
        Err(e) => {
            eprintln!("Erro ao buscar categorias: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

// Criar categoria
#[post("/categories", data = "<category_data>")]
pub async fn create_category(category_data: Json<CreateCategoryRequest>) -> Result<Json<ApiResponse<Category>>, Status> {
    let pool = get_pool();
    let category = &category_data.into_inner();

    match sqlx::query_as::<_, Category>(
        "INSERT INTO categories (name, description) VALUES ($1, $2) RETURNING *"
    )
    .bind(&category.name)
    .bind(&category.description)
    .fetch_one(pool)
    .await
    {
        Ok(new_category) => Ok(Json(ApiResponse::success(new_category, "Categoria criada com sucesso"))),
        Err(sqlx::Error::Database(db_err)) if db_err.constraint().is_some() => {
            Err(Status::Conflict) // Nome duplicado
        }
        Err(e) => {
            eprintln!("Erro ao criar categoria: {}", e);
            Err(Status::InternalServerError)
        }
    }
}
