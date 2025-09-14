#!/bin/bash

# Script para testar a API do aplicativo Rocket
# Execute este script após iniciar o servidor com 'cargo run'

BASE_URL="http://localhost:8000"

echo "=== Testando API Rocket + PostgreSQL ==="
echo ""

# Função para fazer requisições e mostrar resultados
test_endpoint() {
    local method=$1
    local endpoint=$2
    local data=$3
    local description=$4
    
    echo "🔍 $description"
    echo "   $method $endpoint"
    
    if [ -n "$data" ]; then
        response=$(curl -s -X $method "$BASE_URL$endpoint" \
            -H "Content-Type: application/json" \
            -d "$data")
    else
        response=$(curl -s -X $method "$BASE_URL$endpoint")
    fi
    
    echo "$response" | jq . 2>/dev/null || echo "$response"
    echo ""
}

# Verificar se o servidor está rodando
echo "Verificando se o servidor está rodando..."
if ! curl -s "$BASE_URL" > /dev/null; then
    echo "❌ Serviço não está rodando!"
    echo "Execute 'cargo run' primeiro"
    exit 1
fi

echo "✅ Servidor está rodando"
echo ""

# Teste 1: Página inicial
test_endpoint "GET" "/" "" "Página inicial"

# Teste 2: Listar usuários (deve estar vazio inicialmente)
test_endpoint "GET" "/users" "" "Listar usuários (inicial)"

# Teste 3: Criar primeiro usuário
test_endpoint "POST" "/users" '{
    "name": "João Silva",
    "email": "joao@email.com",
    "age": 30
}' "Criar primeiro usuário"

# Teste 4: Criar segundo usuário
test_endpoint "POST" "/users" '{
    "name": "Maria Santos",
    "email": "maria@email.com",
    "age": 25
}' "Criar segundo usuário"

# Teste 5: Listar usuários (agora deve ter 2)
test_endpoint "GET" "/users" "" "Listar usuários (após criação)"

# Teste 6: Buscar usuário específico (usar ID do primeiro usuário)
echo "🔍 Buscar usuário específico"
echo "   GET /users/{id}"
echo "   (Execute manualmente com um ID válido da listagem anterior)"
echo ""

# Teste 7: Atualizar usuário
echo "🔍 Atualizar usuário"
echo "   PUT /users/{id}"
echo "   (Execute manualmente com um ID válido)"
echo ""

# Teste 8: Deletar usuário
echo "🔍 Deletar usuário"
echo "   DELETE /users/{id}"
echo "   (Execute manualmente com um ID válido)"
echo ""

echo "✅ Testes básicos concluídos!"
echo ""
echo "Para testar endpoints específicos, use:"
echo "  curl -X GET $BASE_URL/users"
echo "  curl -X POST $BASE_URL/users -H 'Content-Type: application/json' -d '{\"name\":\"Teste\",\"email\":\"teste@email.com\",\"age\":20}'"
