#!/bin/bash

# Script para testar a funcionalidade de livros
# Execute este script após iniciar o servidor com 'cargo run'

BASE_URL="http://localhost:8000"

echo "=== Testando Sistema de Livros ==="
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

echo "✅ Serviço está rodando"
echo ""

# Teste 1: Página de livros
echo "🔍 Acessando página de livros"
curl -s "$BASE_URL/library" | grep -q "Biblioteca Digital" && echo "✅ Página de livros carregada" || echo "❌ Erro na página de livros"
echo ""

# Teste 2: Listar categorias
test_endpoint "GET" "/categories" "" "Listar categorias disponíveis"

# Teste 3: Criar uma categoria
test_endpoint "POST" "/categories" '{
    "name": "Teste",
    "description": "Categoria para testes"
}' "Criar nova categoria"

# Teste 4: Listar livros (deve estar vazio inicialmente)
test_endpoint "GET" "/books" "" "Listar livros (inicial)"

# Teste 5: Criar primeiro livro (usando category_id da lista anterior)
test_endpoint "POST" "/books" '{
    "title": "O Guia do Mochileiro das Galáxias",
    "author": "Douglas Adams",
    "isbn": "978-0-345-39180-3",
    "description": "Uma comédia de ficção científica sobre as aventuras de Arthur Dent no espaço.",
    "content": "Capítulo 1: A Casa\n\nArthur Dent estava deitado na grama em frente à sua casa, olhando para o céu e pensando em como sua vida estava prestes a mudar para sempre. Ele não sabia que em poucos minutos sua casa seria demolida para dar lugar a uma rodovia interestelar.\n\nCapítulo 2: A Demolição\n\nQuando os tratores chegaram, Arthur tentou impedir a demolição, mas foi informado que a ordem vinha diretamente do Conselho Galáctico. Foi então que seu amigo Ford Prefect revelou que era na verdade um alienígena e que a Terra estava prestes a ser destruída.",
    "category_id": "5a9a0db4-4b83-4031-a44a-d5f7fc2f43cc",
    "is_public": true
}' "Criar primeiro livro"

# Teste 6: Criar segundo livro
test_endpoint "POST" "/books" '{
    "title": "1984",
    "author": "George Orwell",
    "isbn": "978-0-452-28423-4",
    "description": "Um romance distópico sobre totalitarismo e controle social.",
    "content": "Capítulo 1: O Grande Irmão\n\nEra um dia frio e brilhante de abril, e os relógios batiam treze. Winston Smith, com o queixo colado ao peito para escapar do vento cortante, deslizou rapidamente pelas portas de vidro do Victory Mansions, mas não com rapidez suficiente para impedir que uma rajada de areia e poeira entrasse junto com ele.\n\nO corredor cheirava a repolho cozido e tapetes velhos. Na parede do fundo, um cartaz colorido, muito grande demais para uso interno, estava afixado na parede. Representava simplesmente um rosto enorme, de mais de um metro de largura: o rosto de um homem de uns 45 anos, com bigode grosso e feições bonitas e brutais.",
    "category_id": "00000000-0000-0000-0000-000000000001",
    "is_public": true
}' "Criar segundo livro"

# Teste 7: Listar livros (agora deve ter 2)
test_endpoint "GET" "/books" "" "Listar livros (após criação)"

# Teste 8: Buscar livro específico
echo "🔍 Buscar livro específico"
echo "   GET /books/{id}"
echo "   (Execute manualmente com um ID válido da listagem anterior)"
echo ""

# Teste 9: Atualizar livro
echo "🔍 Atualizar livro"
echo "   PUT /books/{id}"
echo "   (Execute manualmente com um ID válido)"
echo ""

# Teste 10: Deletar livro
echo "🔍 Deletar livro"
echo "   DELETE /books/{id}"
echo "   (Execute manualmente com um ID válido)"
echo ""

echo "✅ Testes de livros concluídos!"
echo ""
echo "Para testar a interface web:"
echo "  1. Acesse: http://localhost:8000/library"
echo "  2. Teste a criação, edição e leitura de livros"
echo "  3. Use a busca por categoria e texto"
echo ""
echo "Para testar com cURL:"
echo "  curl -X POST http://localhost:8000/books -H 'Content-Type: application/json' -d '{\"title\":\"Meu Livro\",\"author\":\"Autor\",\"content\":\"Conteúdo\",\"category_id\":\"CATEGORY_ID\",\"is_public\":true}'"
