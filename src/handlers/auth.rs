use rocket::{post, http::Status, serde::json::Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::{
    models::{LoginRequest, RegisterRequest, LoginResponse, User, UserResponse, ApiResponse, JwtConfig},
    database::get_pool,
};

// Endpoint de login
#[post("/login", data = "<login_data>")]
pub async fn login(login_data: Json<LoginRequest>) -> Result<Json<ApiResponse<LoginResponse>>, Status> {
    let pool = get_pool();
    let login = &login_data.into_inner();

    // Buscar usuário por email
    match sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&login.email)
        .fetch_one(pool)
        .await
    {
        Ok(user) => {
            // Verificar senha
            match verify(&login.password, &user.password_hash) {
                Ok(valid) if valid => {
                    // Gerar token JWT
                    let jwt_config = JwtConfig::new();
                    match jwt_config.generate_token(user.id, &user.email) {
                        Ok(token) => {
                            let login_response = LoginResponse {
                                token,
                                user: UserResponse::from(user),
                            };
                            Ok(Json(ApiResponse::success(login_response, "Login realizado com sucesso")))
                        }
                        Err(_) => Err(Status::InternalServerError)
                    }
                }
                Ok(_) => Err(Status::Unauthorized),
                Err(_) => Err(Status::InternalServerError)
            }
        }
        Err(sqlx::Error::RowNotFound) => Err(Status::Unauthorized),
        Err(e) => {
            eprintln!("Erro ao buscar usuário: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

// Endpoint de registro
#[post("/register", data = "<register_data>")]
pub async fn register(register_data: Json<RegisterRequest>) -> Result<Json<ApiResponse<LoginResponse>>, Status> {
    let pool = get_pool();
    let register = &register_data.into_inner();

    // Hash da senha
    let password_hash = match hash(&register.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => return Err(Status::InternalServerError),
    };

    // Criar usuário
    match sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email, password_hash, age) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&register.name)
    .bind(&register.email)
    .bind(&password_hash)
    .bind(&register.age)
    .fetch_one(pool)
    .await
    {
        Ok(user) => {
            // Gerar token JWT
            let jwt_config = JwtConfig::new();
            match jwt_config.generate_token(user.id, &user.email) {
                Ok(token) => {
                    let login_response = LoginResponse {
                        token,
                        user: UserResponse::from(user),
                    };
                    Ok(Json(ApiResponse::success(login_response, "Usuário registrado com sucesso")))
                }
                Err(_) => Err(Status::InternalServerError)
            }
        }
        Err(sqlx::Error::Database(db_err)) if db_err.constraint().is_some() => {
            Err(Status::Conflict) // Email duplicado
        }
        Err(e) => {
            eprintln!("Erro ao criar usuário: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

// Endpoint para verificar token (protegido)
#[post("/verify-token")]
pub async fn verify_token() -> Result<Json<ApiResponse<UserResponse>>, Status> {
    // Este endpoint seria usado para verificar se o token ainda é válido
    // A implementação completa incluiria middleware de autenticação
    Err(Status::NotImplemented)
}
