#!/bin/bash

# Script para configurar o banco de dados PostgreSQL
# Execute este script antes de rodar o aplicativo

echo "=== Configuração do Banco de Dados PostgreSQL ==="
echo ""

# Verificar se o PostgreSQL está instalado
if ! command -v psql &> /dev/null; then
    echo "❌ PostgreSQL não está instalado!"
    echo "Instale o PostgreSQL primeiro:"
    echo "  Ubuntu/Debian: sudo apt install postgresql postgresql-contrib"
    echo "  Arch Linux: sudo pacman -S postgresql"
    echo "  macOS: brew install postgresql"
    exit 1
fi

echo "✅ PostgreSQL encontrado"

# Verificar se o serviço está rodando
if ! pg_isready -q; then
    echo "❌ Serviço PostgreSQL não está rodando!"
    echo "Inicie o serviço:"
    echo "  sudo systemctl start postgresql"
    echo "  # ou no macOS: brew services start postgresql"
    exit 1
fi

echo "✅ Serviço PostgreSQL está rodando"

# Criar banco de dados
echo ""
echo "Criando banco de dados 'rocket_tutorial'..."

sudo -u postgres psql -c "CREATE DATABASE rocket_tutorial;" 2>/dev/null || {
    echo "⚠️  Banco de dados 'rocket_tutorial' já existe ou erro na criação"
}

# Criar usuário se não existir
echo "Configurando usuário 'postgres'..."

sudo -u postgres psql -c "ALTER USER postgres PASSWORD 'senha123';" 2>/dev/null || {
    echo "⚠️  Erro ao configurar senha do usuário postgres"
}

echo ""
echo "✅ Configuração do banco concluída!"
echo ""
echo "Configurações de conexão:"
echo "  Host: localhost"
echo "  Porta: 5432"
echo "  Banco: rocket_tutorial"
echo "  Usuário: postgres"
echo "  Senha: senha123"
echo ""
echo "Agora você pode executar: cargo run"
