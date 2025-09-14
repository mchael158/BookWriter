# Guia do Git - Rocket + PostgreSQL Tutorial

Este guia explica como usar o Git neste projeto de aplicação de leitura.

## Estrutura do Repositório

### Branches
- `main`: Branch principal com código estável
- `develop`: Branch de desenvolvimento para novas funcionalidades

### Commits
O projeto usa [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` nova funcionalidade
- `fix:` correção de bug
- `docs:` documentação
- `style:` formatação
- `refactor:` refatoração
- `test:` testes
- `chore:` manutenção

## Comandos Úteis

### Configuração Inicial
```bash
# Clone o repositório
git clone <url-do-repositorio>
cd rocket-postgres-tutorial

# Configure seu usuário
git config user.name "Seu Nome"
git config user.email "seu@email.com"
```

### Desenvolvimento
```bash
# Criar nova branch para feature
git checkout -b feature/nova-funcionalidade

# Fazer alterações e commit
git add .
git commit -m "feat: adiciona nova funcionalidade X"

# Push da branch
git push origin feature/nova-funcionalidade
```

### Merge e Deploy
```bash
# Voltar para develop
git checkout develop

# Merge da feature
git merge feature/nova-funcionalidade

# Push para develop
git push origin develop

# Merge para main (quando pronto)
git checkout main
git merge develop
git push origin main
```

### Limpeza
```bash
# Deletar branch local
git branch -d feature/nova-funcionalidade

# Deletar branch remota
git push origin --delete feature/nova-funcionalidade

# Limpar branches órfãs
git remote prune origin
```

## Workflow Recomendado

### 1. Desenvolvimento de Nova Funcionalidade
```bash
# 1. Atualizar develop
git checkout develop
git pull origin develop

# 2. Criar branch da feature
git checkout -b feature/nome-da-funcionalidade

# 3. Desenvolver e commitar
git add .
git commit -m "feat: descrição da funcionalidade"

# 4. Push da branch
git push origin feature/nome-da-funcionalidade

# 5. Criar Pull Request no GitHub
```

### 2. Correção de Bug
```bash
# 1. Atualizar main
git checkout main
git pull origin main

# 2. Criar branch do fix
git checkout -b fix/descricao-do-bug

# 3. Corrigir e commitar
git add .
git commit -m "fix: descrição da correção"

# 4. Push da branch
git push origin fix/descricao-do-bug
```

### 3. Release
```bash
# 1. Atualizar main
git checkout main
git pull origin main

# 2. Criar tag de versão
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0

# 3. Atualizar CHANGELOG.md
# 4. Commit da atualização
git add CHANGELOG.md
git commit -m "docs: atualiza changelog para v1.0.0"
git push origin main
```

## Arquivos Importantes

### .gitignore
Configurado para ignorar:
- Arquivos de build (`target/`)
- Arquivos temporários
- Logs
- Arquivos de IDE
- Arquivos de sistema

### .gitattributes
Configurado para:
- Normalização de line endings
- Tratamento correto de arquivos binários
- Configuração de linguagem para diffs

## Troubleshooting

### Problemas Comuns

#### 1. Merge Conflicts
```bash
# Ver arquivos em conflito
git status

# Resolver conflitos manualmente
# Depois:
git add .
git commit -m "fix: resolve merge conflicts"
```

#### 2. Commit Errado
```bash
# Desfazer último commit (mantendo alterações)
git reset --soft HEAD~1

# Desfazer último commit (perdendo alterações)
git reset --hard HEAD~1

# Alterar mensagem do último commit
git commit --amend -m "nova mensagem"
```

#### 3. Branch Desatualizada
```bash
# Atualizar branch com main
git checkout feature/minha-branch
git rebase main

# Resolver conflitos se houver
# Continuar rebase
git rebase --continue
```

#### 4. Histórico Limpo
```bash
# Squash commits em uma branch
git rebase -i HEAD~3

# Alterar mensagens de commit
git rebase -i HEAD~3
```

## Integração com GitHub

### GitHub Actions
O projeto inclui CI/CD automatizado:
- Testes executados em cada PR
- Build automático em push para main
- Verificação de segurança com cargo-audit
- Formatação e linting automáticos

### Pull Requests
Template sugerido:
```markdown
## Descrição
Breve descrição das mudanças

## Tipo de Mudança
- [ ] Bug fix
- [ ] Nova funcionalidade
- [ ] Breaking change
- [ ] Documentação

## Checklist
- [ ] Código testado
- [ ] Documentação atualizada
- [ ] Testes passando
- [ ] Sem warnings
```

## Comandos de Emergência

### Reset Completo
```bash
# CUIDADO: Remove todas as alterações não commitadas
git reset --hard HEAD
git clean -fd
```

### Recuperar Arquivo
```bash
# Recuperar arquivo do último commit
git checkout HEAD -- caminho/do/arquivo

# Recuperar arquivo de commit específico
git checkout <commit-hash> -- caminho/do/arquivo
```

### Histórico de Arquivo
```bash
# Ver histórico de um arquivo
git log --follow -- caminho/do/arquivo

# Ver mudanças em um arquivo
git log -p -- caminho/do/arquivo
```

## Dicas e Boas Práticas

1. **Commits Pequenos**: Faça commits frequentes e pequenos
2. **Mensagens Descritivas**: Use mensagens claras e descritivas
3. **Branch Limpa**: Mantenha branches organizadas
4. **Testes**: Sempre teste antes de fazer push
5. **Documentação**: Atualize documentação quando necessário
6. **Review**: Sempre revise código antes de merge
7. **Backup**: Faça backup de branches importantes

## Recursos Adicionais

- [Documentação oficial do Git](https://git-scm.com/doc)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [GitHub Flow](https://guides.github.com/introduction/flow/)
- [Git Best Practices](https://github.com/git/git/blob/master/Documentation/SubmittingPatches)
