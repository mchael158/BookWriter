# Exemplos de Uso da API

Este arquivo contém exemplos práticos de como usar a API do aplicativo Rocket + PostgreSQL.

## Configuração Inicial

1. **Instalar dependências:**
   ```bash
   cargo build
   ```

2. **Configurar banco de dados:**
   ```bash
   ./setup_database.sh
   ```

3. **Iniciar servidor:**
   ```bash
   cargo run
   ```

## Exemplos com cURL

### 1. Verificar se o servidor está rodando
```bash
curl http://localhost:8000/
```

### 2. Listar todos os usuários
```bash
curl http://localhost:8000/users
```

### 3. Criar um novo usuário
```bash
curl -X POST http://localhost:8000/users \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Ana Costa",
    "email": "ana@email.com",
    "age": 28
  }'
```

### 4. Buscar usuário por ID
```bash
# Substitua {id} pelo ID real retornado na criação
curl http://localhost:8000/users/123e4567-e89b-12d3-a456-426614174000
```

### 5. Atualizar usuário
```bash
curl -X PUT http://localhost:8000/users/123e4567-e89b-12d3-a456-426614174000 \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Ana Costa Silva",
    "age": 29
  }'
```

### 6. Deletar usuário
```bash
curl -X DELETE http://localhost:8000/users/123e4567-e89b-12d3-a456-426614174000
```

## Exemplos com JavaScript (Frontend)

### Criar usuário via JavaScript
```javascript
async function criarUsuario() {
    const response = await fetch('http://localhost:8000/users', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            name: 'João Silva',
            email: 'joao@email.com',
            age: 30
        })
    });
    
    const data = await response.json();
    console.log('Usuário criado:', data);
}
```

### Listar usuários
```javascript
async function listarUsuarios() {
    const response = await fetch('http://localhost:8000/users');
    const data = await response.json();
    console.log('Usuários:', data);
}
```

## Exemplos com Python

### Usando requests
```python
import requests
import json

# Criar usuário
def criar_usuario():
    url = 'http://localhost:8000/users'
    data = {
        'name': 'Maria Santos',
        'email': 'maria@email.com',
        'age': 25
    }
    
    response = requests.post(url, json=data)
    return response.json()

# Listar usuários
def listar_usuarios():
    url = 'http://localhost:8000/users'
    response = requests.get(url)
    return response.json()

# Exemplo de uso
if __name__ == '__main__':
    # Criar usuário
    novo_usuario = criar_usuario()
    print('Usuário criado:', novo_usuario)
    
    # Listar usuários
    usuarios = listar_usuarios()
    print('Lista de usuários:', usuarios)
```

## Testando com o Script Automatizado

Execute o script de testes para verificar se tudo está funcionando:

```bash
./test_api.sh
```

## Estrutura das Respostas

### Resposta de Sucesso
```json
{
  "success": true,
  "message": "Usuário criado com sucesso",
  "data": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "name": "João Silva",
    "email": "joao@email.com",
    "age": 30,
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": "2024-01-15T10:30:00Z"
  }
}
```

### Resposta de Erro
```json
{
  "success": false,
  "message": "Usuário não encontrado",
  "data": null
}
```

## Códigos de Status HTTP

- `200 OK`: Operação realizada com sucesso
- `201 Created`: Recurso criado com sucesso
- `400 Bad Request`: Dados inválidos na requisição
- `404 Not Found`: Recurso não encontrado
- `409 Conflict`: Conflito (ex: email duplicado)
- `500 Internal Server Error`: Erro interno do servidor

## Dicas de Uso

1. **Sempre use Content-Type: application/json** para requisições POST e PUT
2. **Os IDs são UUIDs** - use o formato correto
3. **Campos opcionais** podem ser omitidos nas requisições
4. **Email deve ser único** - não pode haver duplicatas
5. **Use o script de teste** para verificar se tudo está funcionando

## Solução de Problemas Comuns

### Erro de Conexão
- Verifique se o servidor está rodando: `cargo run`
- Confirme a porta: `http://localhost:8000`

### Erro de Banco de Dados
- Execute: `./setup_database.sh`
- Verifique se o PostgreSQL está rodando

### Erro de JSON
- Use aspas duplas nos JSONs
- Verifique se o Content-Type está correto
