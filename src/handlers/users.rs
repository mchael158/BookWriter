use rocket::{get, post, put, delete, http::Status, serde::json::Json};
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};
use crate::{
    models::{User, CreateUserRequest, UpdateUserRequest, ApiResponse, UserResponse},
    database::get_pool,
};

// Listar todos os usuários
#[get("/users")]
pub async fn get_users() -> Result<Json<ApiResponse<Vec<UserResponse>>>, Status> {
    let pool = get_pool();
    
    match sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(pool)
        .await
    {
        Ok(users) => {
            let user_responses: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();
            Ok(Json(ApiResponse::success(user_responses, "Usuários listados com sucesso")))
        },
        Err(e) => {
            eprintln!("Erro ao buscar usuários: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

// Buscar usuário por ID
#[get("/users/<id>")]
// #[get("/users/<Name>")]
pub async fn get_user(id: String) -> Result<Json<ApiResponse<UserResponse>>, Status> {
    let pool = get_pool();
    let user_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return Err(Status::BadRequest),
    };

    match sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await
    {
        Ok(user) => Ok(Json(ApiResponse::success(UserResponse::from(user), "Usuário encontrado"))),
        Err(sqlx::Error::RowNotFound) => Err(Status::NotFound),
        Err(e) => {
            eprintln!("Erro ao buscar usuário: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

// Criar novo usuário
#[post("/users", data = "<user_data>")]
pub async fn create_user(user_data: Json<CreateUserRequest>) -> Result<Json<ApiResponse<UserResponse>>, Status> {
    let pool = get_pool();
    let user = &user_data.into_inner();

    // Hash da senha
    let password_hash = match hash(&user.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => return Err(Status::InternalServerError),
    };

    match sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email, password_hash, age) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&user.name)
    .bind(&user.email)
    .bind(&password_hash)
    .bind(&user.age)
    .fetch_one(pool)
    .await
    {
        Ok(new_user) => Ok(Json(ApiResponse::success(UserResponse::from(new_user), "Usuário criado com sucesso"))),
        Err(sqlx::Error::Database(db_err)) if db_err.constraint().is_some() => {
            Err(Status::Conflict) // Email duplicado
        }
        Err(e) => {
            eprintln!("Erro ao criar usuário: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

// Atualizar usuário
#[put("/users/<id>", data = "<user_data>")]
pub async fn update_user(id: String, user_data: Json<UpdateUserRequest>) -> Result<Json<ApiResponse<User>>, Status> {
    let pool = get_pool();
    let user_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return Err(Status::BadRequest),
    };

    let user = &user_data.into_inner();

    // Construir query dinamicamente baseada nos campos fornecidos
    let mut query = "UPDATE users SET ".to_string();
    let mut params: Vec<String> = Vec::new();
    let mut param_count = 1;

    if let Some(ref _name) = user.name {
        params.push(format!("name = ${}", param_count));
        param_count += 1;
    }
    if let Some(ref _email) = user.email {
        params.push(format!("email = ${}", param_count));
        param_count += 1;
    }
    if let Some(_age) = user.age {
        params.push(format!("age = ${}", param_count));
        param_count += 1;
    }

    if params.is_empty() {
        return Err(Status::BadRequest);
    }

    params.push(format!("updated_at = NOW()"));
    query.push_str(&params.join(", "));
    query.push_str(&format!(" WHERE id = ${} RETURNING *", param_count));

    let mut query_builder = sqlx::query_as::<_, User>(&query);
    
    if let Some(ref name) = user.name {
        query_builder = query_builder.bind(name);
    }
    if let Some(ref email) = user.email {
        query_builder = query_builder.bind(email);
    }
    if let Some(age) = user.age {
        query_builder = query_builder.bind(age);
    }
    query_builder = query_builder.bind(user_id);

    match query_builder.fetch_one(pool).await {
        Ok(updated_user) => Ok(Json(ApiResponse::success(updated_user, "Usuário atualizado com sucesso"))),
        Err(sqlx::Error::RowNotFound) => Err(Status::NotFound),
        Err(sqlx::Error::Database(db_err)) if db_err.constraint().is_some() => {
            Err(Status::Conflict) // Email duplicado
        }
        Err(e) => {
            eprintln!("Erro ao atualizar usuário: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

// Deletar usuário
#[delete("/users/<id>")]
pub async fn delete_user(id: String) -> Result<Json<ApiResponse<()>>, Status> {
    let pool = get_pool();
    let user_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return Err(Status::BadRequest),
    };

    match sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => {
            Ok(Json(ApiResponse::success((), "Usuário deletado com sucesso")))
        }
        Ok(_) => Err(Status::NotFound),
        Err(e) => {
            eprintln!("Erro ao deletar usuário: {}", e);
            Err(Status::InternalServerError)
        }
    }
}
