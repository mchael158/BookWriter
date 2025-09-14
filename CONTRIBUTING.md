# Guia de Contribuição

Obrigado por considerar contribuir para este projeto! Este documento fornece diretrizes para contribuições.

## Como Contribuir

### 1. Fork do Projeto
- Faça um fork do repositório
- Clone seu fork localmente
- Configure o upstream para o repositório original

### 2. Configuração do Ambiente
```bash
# Clone o repositório
git clone https://github.com/seu-usuario/rocket-postgres-tutorial.git
cd rocket-postgres-tutorial

# Configure o banco de dados
./setup_database.sh

# Instale as dependências
cargo build

# Execute os testes
./test_api.sh
./test_auth.sh
./test_books.sh
```

### 3. Criando uma Branch
```bash
# Crie uma branch para sua feature
git checkout -b feature/nova-funcionalidade

# Ou para correção de bugs
git checkout -b fix/correcao-bug
```

### 4. Desenvolvimento
- Faça suas alterações
- Siga as convenções de código existentes
- Adicione testes para novas funcionalidades
- Atualize a documentação conforme necessário

### 5. Testes
```bash
# Execute todos os testes
cargo test

# Execute os scripts de teste
./test_api.sh
./test_auth.sh
./test_books.sh

# Verifique se não há warnings
cargo check
```

### 6. Commit
```bash
# Adicione suas alterações
git add .

# Faça um commit descritivo
git commit -m "feat: adiciona nova funcionalidade X"
```

### 7. Push e Pull Request
```bash
# Push para sua branch
git push origin feature/nova-funcionalidade

# Crie um Pull Request no GitHub
```

## Convenções de Código

### Rust
- Use `cargo fmt` para formatação
- Use `cargo clippy` para linting
- Siga as convenções do Rust
- Documente funções públicas

### Commits
Use o formato conventional commits:
- `feat:` nova funcionalidade
- `fix:` correção de bug
- `docs:` documentação
- `style:` formatação
- `refactor:` refatoração
- `test:` testes
- `chore:` manutenção

### Exemplos
```bash
git commit -m "feat: adiciona sistema de favoritos"
git commit -m "fix: corrige erro de validação de email"
git commit -m "docs: atualiza README com novas instruções"
```

## Estrutura do Projeto

```
src/
├── main.rs              # Ponto de entrada
├── database.rs          # Configuração do banco
├── models.rs            # Modelos principais
├── models/book.rs       # Modelos de livros
└── handlers/            # Handlers da API
    ├── mod.rs
    ├── auth.rs          # Autenticação
    ├── users.rs         # Usuários
    └── books.rs         # Livros
```

## Reportando Bugs

1. Verifique se o bug já foi reportado
2. Use o template de issue
3. Inclua informações detalhadas:
   - Versão do Rust
   - Sistema operacional
   - Passos para reproduzir
   - Logs de erro
   - Screenshots (se aplicável)

## Sugerindo Funcionalidades

1. Verifique se a funcionalidade já foi sugerida
2. Use o template de feature request
3. Descreva claramente a funcionalidade
4. Explique o caso de uso
5. Considere a implementação

## Perguntas e Discussões

- Use as Discussions do GitHub para perguntas
- Seja respeitoso e construtivo
- Ajude outros contribuidores
- Compartilhe conhecimento

## Licença

Ao contribuir, você concorda que suas contribuições serão licenciadas sob a licença MIT.

## Agradecimentos

Obrigado por contribuir para este projeto! Suas contribuições são muito valorizadas.
