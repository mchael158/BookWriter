# Tutorial Completo: Aplicativo Web com Rocket + PostgreSQL

Este é um tutorial completo e detalhado para criar um aplicativo web usando o framework Rocket (Rust) com banco de dados PostgreSQL. O projeto inclui uma API REST completa para gerenciamento de usuários.

## Pré-requisitos

Antes de começar, certifique-se de ter instalado:

1. **Rust** (versão 1.70+)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **PostgreSQL** (versão 12+)
   - Ubuntu/Debian: `sudo apt install postgresql postgresql-contrib`
   - Arch Linux: `sudo pacman -S postgresql`
   - macOS: `brew install postgresql`

3. **Ferramentas de desenvolvimento**
   - `jq` (para formatação JSON): `sudo apt install jq`
   - `curl` (para testes de API)

## Estrutura do Projeto

```
rocket-postgres-tutorial/
├── Cargo.toml                 # Dependências do projeto
├── src/
│   ├── main.rs               # Ponto de entrada da aplicação
│   ├── database.rs           # Configuração do banco de dados
│   ├── models.rs             # Modelos de dados e DTOs
│   └── handlers/
│       ├── mod.rs            # Módulo de handlers
│       └── users.rs          # Handlers para usuários
├── templates/
│   └── index.html.hbs        # Template da página inicial
├── setup_database.sh         # Script de configuração do banco
├── test_api.sh              # Script de testes da API
└── README.md                # Esta documentação
```

## Configuração Inicial

### 1. Configurar o Banco de Dados

Execute o script de configuração:

```bash
./setup_database.sh
```

Este script irá:
- Verificar se o PostgreSQL está instalado e rodando
- Criar o banco de dados `rocket_tutorial`
- Configurar o usuário `postgres` com senha `senha123`

### 2. Instalar Dependências

```bash
cargo build
```

### 3. Executar o Aplicativo

```bash
cargo run
```

O servidor estará disponível em: `http://localhost:8000`

## Explicação Detalhada do Código

### 1. Cargo.toml - Dependências

```toml
[dependencies]
rocket = { version = "0.5", features = ["json"] }           # Framework web
rocket_dyn_templates = { version = "0.1", features = ["handlebars"] }  # Templates
serde = { version = "1.0", features = ["derive"] }          # Serialização
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "chrono", "uuid"] }  # Banco de dados
chrono = { version = "0.4", features = ["serde"] }          # Manipulação de datas
uuid = { version = "1.0", features = ["v4", "serde"] }      # Geração de UUIDs
anyhow = "1.0"                                              # Tratamento de erros
```

### 2. main.rs - Ponto de Entrada

O arquivo principal configura:
- Inicialização do banco de dados
- Registro das rotas
- Configuração dos templates
- Lançamento do servidor Rocket

### 3. database.rs - Configuração do Banco

Este módulo gerencia:
- Conexão com PostgreSQL
- Pool de conexões para performance
- Execução de migrações automáticas
- Criação da tabela `users`

### 4. models.rs - Modelos de Dados

Define as estruturas de dados:
- `User`: Modelo principal do banco de dados
- `CreateUserRequest`: DTO para criação de usuários
- `UpdateUserRequest`: DTO para atualização de usuários
- `ApiResponse`: Resposta padronizada da API

### 5. handlers/users.rs - Lógica de Negócio

Implementa os endpoints da API:
- `GET /users`: Lista todos os usuários
- `GET /users/{id}`: Busca usuário por ID
- `POST /users`: Cria novo usuário
- `PUT /users/{id}`: Atualiza usuário existente
- `DELETE /users/{id}`: Remove usuário

## API Endpoints

### Autenticação

#### Registrar Usuário
```bash
curl -X POST http://localhost:8000/register \
  -H "Content-Type: application/json" \
  -d '{
    "name": "João Silva",
    "email": "joao@email.com",
    "password": "senha123",
    "age": 30
  }'
```

#### Fazer Login
```bash
curl -X POST http://localhost:8000/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "joao@email.com",
    "password": "senha123"
  }'
```

### Biblioteca Digital

#### Listar Livros
```bash
curl http://localhost:8000/books
```

#### Buscar Livro por ID
```bash
curl http://localhost:8000/books/{id}
```

#### Criar Livro
```bash
curl -X POST http://localhost:8000/books \
  -H "Content-Type: application/json" \
  -d '{
    "title": "O Guia do Mochileiro das Galáxias",
    "author": "Douglas Adams",
    "isbn": "978-0-345-39180-3",
    "description": "Uma comédia de ficção científica",
    "content": "Capítulo 1: A Casa...",
    "category_id": "CATEGORY_ID",
    "is_public": true
  }'
```

#### Atualizar Livro
```bash
curl -X PUT http://localhost:8000/books/{id} \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Novo Título",
    "description": "Nova descrição"
  }'
```

#### Deletar Livro
```bash
curl -X DELETE http://localhost:8000/books/{id}
```

#### Listar Categorias
```bash
curl http://localhost:8000/categories
```

#### Criar Categoria
```bash
curl -X POST http://localhost:8000/categories \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Ficção Científica",
    "description": "Livros de ficção científica"
  }'
```

### Gerenciamento de Usuários

#### Listar Usuários
```bash
curl http://localhost:8000/users
```

#### Buscar Usuário por ID
```bash
curl http://localhost:8000/users/{id}
```

#### Criar Usuário
```bash
curl -X POST http://localhost:8000/users \
  -H "Content-Type: application/json" \
  -d '{
    "name": "João Silva",
    "email": "joao@email.com",
    "password": "senha123",
    "age": 30
  }'
```

#### Atualizar Usuário
```bash
curl -X PUT http://localhost:8000/users/{id} \
  -H "Content-Type: application/json" \
  -d '{
    "name": "João Silva Atualizado",
    "age": 31
  }'
```

#### Deletar Usuário
```bash
curl -X DELETE http://localhost:8000/users/{id}
```

## Testando a API

Execute os scripts de testes:

```bash
# Testar API básica
./test_api.sh

# Testar sistema de autenticação
./test_auth.sh

# Testar funcionalidades de livros
./test_books.sh
```

Estes scripts testam todos os endpoints automaticamente e mostram os resultados.

## Interface Web

O aplicativo inclui uma interface web completa:

- **Página Inicial**: http://localhost:8000
- **Login**: http://localhost:8000/login
- **Registro**: http://localhost:8000/register
- **Dashboard**: http://localhost:8000/dashboard
- **Biblioteca Digital**: http://localhost:8000/library

## Estrutura do Banco de Dados

### Tabela de Usuários
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    age INTEGER,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### Tabela de Categorias
```sql
CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

### Tabela de Livros
```sql
CREATE TABLE books (
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
);
```

### Tabela de Progresso de Leitura
```sql
CREATE TABLE reading_progress (
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
);
```

## Recursos Implementados

### Segurança
- Autenticação JWT com tokens seguros
- Hash de senhas com bcrypt
- Validação de entrada de dados
- Tratamento de erros robusto
- Prevenção de SQL injection (usando SQLx)
- Validação de UUIDs

### Performance
- Pool de conexões com banco de dados
- Queries otimizadas
- Serialização eficiente com Serde

### Usabilidade
- Interface web moderna e responsiva
- Sistema de login e registro completo
- Dashboard interativo para usuários
- Biblioteca digital com funcionalidades completas
- Sistema de categorias para organização
- Interface de leitura de livros
- Documentação interativa
- Scripts de automação
- Respostas padronizadas da API

## Próximos Passos

Para expandir este projeto, você pode:

1. **Adicionar autenticação**
   - Implementar JWT tokens
   - Sistema de login/logout
   - Middleware de autenticação

2. **Melhorar a interface**
   - Adicionar formulários interativos
   - Implementar JavaScript para AJAX
   - Adicionar validação no frontend

3. **Adicionar mais funcionalidades**
   - Paginação de resultados
   - Filtros e busca
   - Upload de arquivos
   - Logs de auditoria

4. **Deploy em produção**
   - Configurar Docker
   - Usar variáveis de ambiente
   - Configurar HTTPS
   - Implementar monitoramento

## Solução de Problemas

### Erro de Conexão com Banco
- Verifique se o PostgreSQL está rodando: `sudo systemctl status postgresql`
- Confirme as credenciais no arquivo `database.rs`
- Execute o script `setup_database.sh` novamente

### Erro de Compilação
- Atualize o Rust: `rustup update`
- Limpe o cache: `cargo clean`
- Recompile: `cargo build`

### Erro de Porta em Uso
- Mude a porta no arquivo `main.rs`
- Ou mate o processo: `sudo lsof -ti:8000 | xargs kill -9`

## Conclusão

Este tutorial demonstra como criar um aplicativo web completo usando Rust com Rocket e PostgreSQL. O código é seguro, performático e fácil de manter, seguindo as melhores práticas da linguagem Rust.

O projeto serve como base sólida para aplicações web mais complexas e pode ser facilmente expandido conforme suas necessidades.
