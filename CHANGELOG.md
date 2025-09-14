# Changelog

Todas as mudanças notáveis neste projeto serão documentadas neste arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/),
e este projeto adere ao [Versionamento Semântico](https://semver.org/lang/pt-BR/).

## [0.1.0] - 2024-01-15

### Adicionado
- 🎉 Commit inicial do projeto
- ✨ Sistema de autenticação JWT completo
- 🔐 Hash de senhas com bcrypt
- 👤 Sistema de usuários com CRUD completo
- 📚 Biblioteca digital com funcionalidades completas
- 🏷️ Sistema de categorias predefinidas
- 📖 Interface de leitura de livros
- 🎨 Templates HTML modernos e responsivos
- 🗄️ Banco de dados PostgreSQL com migrações automáticas
- 🔌 API REST completa com todos os endpoints
- 🧪 Scripts de teste automatizados
- 📝 Documentação completa e detalhada
- 🚀 Configuração de desenvolvimento pronta

### Funcionalidades
- **Autenticação**: Login, registro e logout com JWT
- **Usuários**: CRUD completo de usuários
- **Livros**: Criar, editar, deletar e listar livros
- **Categorias**: Sistema de categorias para organização
- **Interface**: Design moderno e responsivo
- **API**: Endpoints REST para todas as funcionalidades
- **Banco**: Migrações automáticas e relacionamentos
- **Testes**: Scripts automatizados para validação

### Tecnologias
- **Backend**: Rocket (Rust)
- **Banco de Dados**: PostgreSQL
- **ORM**: SQLx
- **Autenticação**: JWT + bcrypt
- **Templates**: Handlebars
- **Frontend**: HTML5 + CSS3 + JavaScript

### Estrutura do Projeto
```
rocket-postgres-tutorial/
├── src/                    # Código fonte Rust
│   ├── main.rs            # Ponto de entrada
│   ├── database.rs        # Configuração do banco
│   ├── models.rs          # Modelos de dados
│   ├── models/book.rs     # Modelos de livros
│   └── handlers/          # Handlers da API
├── templates/             # Templates HTML
├── scripts/              # Scripts de automação
├── docs/                 # Documentação
└── tests/                # Testes automatizados
```

### Próximas Versões
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
