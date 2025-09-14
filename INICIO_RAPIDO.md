# Início Rápido - Rocket + PostgreSQL

## Execução em 3 Passos

### 1. Configurar Banco de Dados
```bash
./setup_database.sh
```

### 2. Compilar Projeto
```bash
cargo build
```

### 3. Executar Aplicação
```bash
cargo run
```

## Acessar Aplicação

- **Interface Web**: http://localhost:8000
- **API Usuários**: http://localhost:8000/users

## Testar API

```bash
./test_api.sh
```

## Estrutura Criada

```
rocket-postgres-tutorial/
├── src/                    # Código fonte Rust
├── templates/              # Templates HTML
├── Cargo.toml             # Dependências
├── setup_database.sh      # Script de configuração
├── test_api.sh           # Script de testes
├── README.md             # Documentação completa
└── exemplos_uso.md       # Exemplos práticos
```

## Próximos Passos

1. Leia o `README.md` para entender o código
2. Consulte `exemplos_uso.md` para casos práticos
3. Modifique o código conforme suas necessidades
4. Adicione novas funcionalidades

## Suporte

- Documentação Rocket: https://rocket.rs
- Documentação SQLx: https://docs.rs/sqlx
- PostgreSQL: https://www.postgresql.org/docs
