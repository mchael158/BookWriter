#!/bin/bash

# Script para testar o sistema de autenticação
# Execute este script após iniciar o servidor com 'cargo run'

BASE_URL="http://localhost:8000"

echo "=== Testando Sistema de Autenticação ==="
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

# Teste 1: Página de login
echo "🔍 Acessando página de login"
curl -s "$BASE_URL/login" | grep -q "Entrar" && echo "✅ Página de login carregada" || echo "❌ Erro na página de login"
echo ""

# Teste 2: Página de registro
echo "🔍 Acessando página de registro"
curl -s "$BASE_URL/register" | grep -q "Criar Conta" && echo "✅ Página de registro carregada" || echo "❌ Erro na página de registro"
echo ""

# Teste 3: Página de dashboard
echo "🔍 Acessando página de dashboard"
curl -s "$BASE_URL/dashboard" | grep -q "Dashboard" && echo "✅ Página de dashboard carregada" || echo "❌ Erro na página de dashboard"
echo ""

# Teste 4: Registrar novo usuário
test_endpoint "POST" "/register" '{
    "name": "João Silva",
    "email": "joao@email.com",
    "password": "senha123",
    "age": 30
}' "Registrar novo usuário"

# Teste 5: Fazer login
test_endpoint "POST" "/login" '{
    "email": "joao@email.com",
    "password": "senha123"
}' "Fazer login"

# Teste 6: Tentar login com senha errada
test_endpoint "POST" "/login" '{
    "email": "joao@email.com",
    "password": "senha_errada"
}' "Tentar login com senha errada"

# Teste 7: Tentar registrar usuário com email duplicado
test_endpoint "POST" "/register" '{
    "name": "João Silva Duplicado",
    "email": "joao@email.com",
    "password": "outrasenha",
    "age": 25
}' "Tentar registrar usuário com email duplicado"

# Teste 8: Listar usuários (deve incluir o usuário criado)
test_endpoint "GET" "/users" "" "Listar usuários (deve incluir o usuário criado)"

echo "✅ Testes de autenticação concluídos!"
echo ""
echo "Para testar a interface web:"
echo "  1. Acesse: http://localhost:8000"
echo "  2. Clique em 'Criar Conta' ou 'Fazer Login'"
echo "  3. Teste o dashboard em: http://localhost:8000/dashboard"
echo ""
echo "Para testar com cURL:"
echo "  curl -X POST http://localhost:8000/register -H 'Content-Type: application/json' -d '{\"name\":\"Teste\",\"email\":\"teste@email.com\",\"password\":\"senha123\"}'"
echo "  curl -X POST http://localhost:8000/login -H 'Content-Type: application/json' -d '{\"email\":\"teste@email.com\",\"password\":\"senha123\"}'"
