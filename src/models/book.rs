use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// Modelo de categoria de livro
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Modelo de livro para o banco de dados
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: Option<String>,
    pub description: Option<String>,
    pub content: String,
    pub category_id: Uuid,
    pub user_id: Uuid, // Usuário que criou o livro
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Modelo de livro com informações da categoria
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BookWithCategory {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: Option<String>,
    pub description: Option<String>,
    pub content: String,
    pub category_id: Uuid,
    pub user_id: Uuid,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Campos da categoria
    pub cat_id: Uuid,
    pub cat_name: String,
    pub cat_description: Option<String>,
    pub cat_created_at: DateTime<Utc>,
    pub cat_updated_at: DateTime<Utc>,
}

impl BookWithCategory {
    #[allow(dead_code)]
    pub fn category(&self) -> Category {
        Category {
            id: self.cat_id,
            name: self.cat_name.clone(),
            description: self.cat_description.clone(),
            created_at: self.cat_created_at,
            updated_at: self.cat_updated_at,
        }
    }
}

// DTO para criação de categoria
#[derive(Debug, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
}

// DTO para atualização de categoria
#[derive(Debug, Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

// DTO para criação de livro
#[derive(Debug, Deserialize)]
pub struct CreateBookRequest {
    pub title: String,
    pub author: String,
    pub isbn: Option<String>,
    pub description: Option<String>,
    pub content: String,
    pub category_id: Uuid,
    pub is_public: bool,
}

// DTO para atualização de livro
#[derive(Debug, Deserialize)]
pub struct UpdateBookRequest {
    pub title: Option<String>,
    pub author: Option<String>,
    pub isbn: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub category_id: Option<Uuid>,
    pub is_public: Option<bool>,
}

// DTO para busca de livros
#[derive(Debug, Deserialize)]
pub struct BookSearchRequest {
    pub query: Option<String>,
    pub category_id: Option<String>, // Mudado para String para FromForm
    pub author: Option<String>,
    pub is_public: Option<bool>,
}

// Estatísticas de leitura do usuário
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ReadingStats {
    pub user_id: Uuid,
    pub total_books: i64,
    pub books_read: i64,
    pub books_created: i64,
    pub total_pages: i64,
    pub favorite_category: Option<String>,
}

// Progresso de leitura de um livro
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ReadingProgress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub book_id: Uuid,
    pub current_page: i32,
    pub total_pages: i32,
    pub is_completed: bool,
    pub last_read_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// DTO para atualizar progresso de leitura
#[derive(Debug, Deserialize)]
pub struct UpdateProgressRequest {
    pub current_page: i32,
    pub is_completed: Option<bool>,
}

// Resposta de busca de livros
#[derive(Debug, Serialize)]
pub struct BookSearchResponse {
    pub books: Vec<BookWithCategory>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}
