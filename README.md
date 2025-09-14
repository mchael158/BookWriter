# 📚 BookWriter - Biblioteca Digital

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Rocket](https://img.shields.io/badge/rocket-0.5-blue.svg)](https://rocket.rs/)
[![PostgreSQL](https://img.shields.io/badge/postgresql-15+-blue.svg)](https://www.postgresql.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

Uma aplicação web moderna de **biblioteca digital** construída com **Rocket** (framework web em Rust) e **PostgreSQL**. O projeto implementa um sistema completo de gerenciamento de livros com autenticação, categorias e interface web responsiva.

## ✨ Funcionalidades

- 🔐 **Sistema de Autenticação JWT** completo
- 👤 **Gerenciamento de Usuários** com CRUD
- 📚 **Biblioteca Digital** com livros e categorias
- 🏷️ **Sistema de Categorias** predefinidas
- 📖 **Interface de Leitura** moderna e responsiva
- 🔍 **Busca e Filtros** de livros
- 🌐 **API REST** completa
- 🗄️ **Banco de Dados** PostgreSQL com migrações
- 🎨 **Templates HTML** com Handlebars
- 🧪 **Testes Automatizados** incluídos
- 🐳 **Docker** pronto para produção
- 🚀 **CI/CD** com GitHub Actions

## 🚀 Início Rápido

### Pré-requisitos

- **Rust** 1.70+
- **PostgreSQL** 15+
- **Docker** (opcional)

### Instalação

1. **Clone o repositório**
   ```bash
   git clone https://github.com/mchael158/BookWriter.git
   cd BookWriter
   ```

2. **Configure o banco de dados**
   ```bash
   ./setup_database.sh
   ```

3. **Execute a aplicação**
   ```bash
   cargo run
   ```

4. **Acesse a aplicação**
   - Web: http://localhost:8000
   - API: http://localhost:8000/api

### Docker (Recomendado)

```bash
# Executar com Docker Compose
docker-compose up

# Acessar a aplicação
# Web: http://localhost:8000
# Adminer: http://localhost:8080
```

## 📖 Documentação

- [**Tutorial Completo**](README.md) - Guia detalhado de desenvolvimento
- [**Início Rápido**](INICIO_RAPIDO.md) - Configuração rápida
- [**Exemplos de Uso**](exemplos_uso.md) - Exemplos da API
- [**Guia do Git**](GIT_GUIDE.md) - Como contribuir
- [**Changelog**](CHANGELOG.md) - Histórico de mudanças

## 🏗️ Arquitetura

### Backend
- **Rocket**: Framework web assíncrono em Rust
- **SQLx**: ORM assíncrono para PostgreSQL
- **JWT**: Autenticação com tokens
- **bcrypt**: Hash de senhas seguro

### Frontend
- **Handlebars**: Templates HTML
- **CSS3**: Design responsivo
- **JavaScript**: Interatividade

### Banco de Dados
- **PostgreSQL**: Banco relacional
- **Migrações**: Automáticas via SQLx
- **Relacionamentos**: Users → Books → Categories

## 🔧 API Endpoints

### Autenticação
- `POST /api/auth/login` - Login
- `POST /api/auth/register` - Registro
- `GET /api/auth/verify` - Verificar token

### Usuários
- `GET /api/users` - Listar usuários
- `GET /api/users/{id}` - Obter usuário
- `POST /api/users` - Criar usuário
- `PUT /api/users/{id}` - Atualizar usuário
- `DELETE /api/users/{id}` - Deletar usuário

### Livros
- `GET /api/books` - Listar livros
- `GET /api/books/{id}` - Obter livro
- `POST /api/books` - Criar livro
- `PUT /api/books/{id}` - Atualizar livro
- `DELETE /api/books/{id}` - Deletar livro

### Categorias
- `GET /api/categories` - Listar categorias
- `POST /api/categories` - Criar categoria

## 🧪 Testes

```bash
# Testes da API
./test_api.sh

# Testes de autenticação
./test_auth.sh

# Testes de livros
./test_books.sh

# Testes unitários
cargo test
```

## 🐳 Docker

### Desenvolvimento
```bash
# Executar todos os serviços
docker-compose up

# Apenas o banco
docker-compose up postgres

# Apenas a aplicação
docker-compose up app
```

### Produção
```bash
# Build da imagem
docker build -t bookwriter .

# Executar container
docker run -p 8000:8000 bookwriter
```

## 🚀 Deploy

### GitHub Actions
O projeto inclui CI/CD automatizado:
- ✅ Testes em cada PR
- ✅ Build automático
- ✅ Verificação de segurança
- ✅ Deploy automático

### Variáveis de Ambiente
```bash
DATABASE_URL=postgresql://user:pass@localhost:5432/bookwriter
ROCKET_ADDRESS=0.0.0.0
ROCKET_PORT=8000
JWT_SECRET=sua_chave_secreta
```

## 🤝 Contribuindo

1. Fork o projeto
2. Crie uma branch (`git checkout -b feature/nova-funcionalidade`)
3. Commit suas mudanças (`git commit -m 'feat: adiciona nova funcionalidade'`)
4. Push para a branch (`git push origin feature/nova-funcionalidade`)
5. Abra um Pull Request

Veja [CONTRIBUTING.md](CONTRIBUTING.md) para mais detalhes.

## 📊 Estrutura do Projeto

```
BookWriter/
├── src/                    # Código fonte Rust
│   ├── main.rs            # Ponto de entrada
│   ├── database.rs        # Configuração do banco
│   ├── models.rs          # Modelos de dados
│   ├── models/book.rs     # Modelos de livros
│   └── handlers/          # Handlers da API
│       ├── auth.rs        # Autenticação
│       ├── users.rs       # Usuários
│       └── books.rs       # Livros
├── templates/             # Templates HTML
├── .github/workflows/     # CI/CD
├── docker-compose.yml     # Orquestração
├── Dockerfile            # Imagem Docker
└── docs/                 # Documentação
```

## 📈 Roadmap

- [ ] Sistema de progresso de leitura
- [ ] Estatísticas de usuário
- [ ] Sistema de favoritos
- [ ] Comentários e avaliações
- [ ] Sistema de recomendações
- [ ] Upload de capas de livros
- [ ] Sistema de busca avançada
- [ ] API de estatísticas
- [ ] Sistema de notificações
- [ ] Modo offline

## 📄 Licença

Este projeto está licenciado sob a [Licença MIT](LICENSE).

## 👥 Autores

- **Rocket Tutorial** - *Desenvolvimento inicial* - [GitHub](https://github.com/rocket-tutorial)

## 🙏 Agradecimentos

- [Rocket](https://rocket.rs/) - Framework web incrível
- [SQLx](https://github.com/launchbadge/sqlx) - ORM assíncrono
- [PostgreSQL](https://www.postgresql.org/) - Banco de dados robusto
- [Rust](https://www.rust-lang.org/) - Linguagem de programação

---

⭐ **Se este projeto te ajudou, considere dar uma estrela!** ⭐
