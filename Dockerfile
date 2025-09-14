# Multi-stage build para otimizar o tamanho da imagem
FROM rust:1.70-slim as builder

# Instalar dependências do sistema
RUN apt-get update && apt-get install -y \
    libpq-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Criar diretório de trabalho
WORKDIR /app

# Copiar arquivos de dependências
COPY Cargo.toml Cargo.lock ./

# Criar um binário dummy para cache das dependências
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copiar código fonte
COPY src ./src
COPY templates ./templates

# Build da aplicação
RUN cargo build --release

# Imagem final minimalista
FROM debian:bookworm-slim

# Instalar dependências de runtime
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Criar usuário não-root
RUN useradd -r -s /bin/false appuser

# Copiar binário da aplicação
COPY --from=builder /app/target/release/rocket-postgres-tutorial /usr/local/bin/
COPY --from=builder /app/templates /app/templates

# Configurar permissões
RUN chown -R appuser:appuser /app
USER appuser

# Expor porta
EXPOSE 8000

# Variáveis de ambiente
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
ENV DATABASE_URL=postgresql://postgres:postgres@postgres:5432/rocket_tutorial

# Comando de inicialização
CMD ["rocket-postgres-tutorial"]
