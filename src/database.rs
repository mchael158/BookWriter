use sqlx::PgPool;
use anyhow::Result;

// Configuração do banco de dados
const DATABASE_URL: &str = "postgresql://postgres:senha123@localhost:5432/rocket_tutorial";

// Pool de conexões global
use std::sync::OnceLock;

static DB_POOL: OnceLock<PgPool> = OnceLock::new();

pub async fn init_db() -> Result<()> {
    // Criar pool de conexões
    let pool = PgPool::connect(DATABASE_URL).await?;
    
    // Executar migrações
    run_migrations(&pool).await?;
    
    // Armazenar pool globalmente
    DB_POOL.set(pool).expect("Pool já foi inicializado");
    
    println!("Banco de dados inicializado com sucesso!");
    Ok(())
}

pub fn get_pool() -> &'static PgPool {
    DB_POOL.get().expect("Pool de banco não inicializado")
}

async fn run_migrations(pool: &PgPool) -> Result<()> {
    // Criar tabela de usuários se não existir
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(100) NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            age INTEGER,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        )
        "#
    )
    .execute(pool)
    .await?;

    // Adicionar coluna password_hash se não existir (para migração de bancos existentes)
    sqlx::query(
        r#"
        ALTER TABLE users 
        ADD COLUMN IF NOT EXISTS password_hash VARCHAR(255) DEFAULT 'senha_padrao_hash'
        "#
    )
    .execute(pool)
    .await?;

    // Criar tabela de categorias
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS categories (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(100) NOT NULL UNIQUE,
            description TEXT,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        )
        "#
    )
    .execute(pool)
    .await?;

    // Criar tabela de livros
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS books (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            title VARCHAR(255) NOT NULL,
            author VARCHAR(255) NOT NULL,
            isbn VARCHAR(20),
            description TEXT,
            content TEXT NOT NULL,
            category_id UUID NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            is_public BOOLEAN DEFAULT false,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        )
        "#
    )
    .execute(pool)
    .await?;

    // Criar tabela de progresso de leitura
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS reading_progress (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            book_id UUID NOT NULL REFERENCES books(id) ON DELETE CASCADE,
            current_page INTEGER DEFAULT 0,
            total_pages INTEGER DEFAULT 0,
            is_completed BOOLEAN DEFAULT false,
            last_read_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            UNIQUE(user_id, book_id)
        )
        "#
    )
    .execute(pool)
    .await?;

    // Inserir categorias padrão
    sqlx::query(
        r#"
        INSERT INTO categories (name, description) VALUES 
        ('Ficção', 'Livros de ficção e literatura'),
        ('Não-Ficção', 'Livros informativos e educativos'),
        ('Tecnologia', 'Livros sobre programação e tecnologia'),
        ('História', 'Livros históricos e biografias'),
        ('Ciência', 'Livros científicos e acadêmicos'),
        ('Autoajuda', 'Livros de desenvolvimento pessoal'),
        ('Romance', 'Livros românticos'),
        ('Mistério', 'Livros de suspense e mistério'),
        ('Fantasia', 'Livros de fantasia e ficção científica'),
        ('Poesia', 'Coleções de poesia')
        ON CONFLICT (name) DO NOTHING
        "#
    )
    .execute(pool)
    .await?;

    println!("Migrações executadas com sucesso!");
    Ok(())
}
