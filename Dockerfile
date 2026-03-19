# ── Stage 1: build ────────────────────────────────────────────────────────────
# rust:latest hoje (2026-03) roda em cima de Debian Trixie.
# Isso importa porque o binario compilado aqui vai linkar contra a glibc do Trixie.
# Se trocar a base do runtime pra outra distro, tem que bater a versao da glibc.
FROM rust:latest AS builder

# protobuf-compiler: o tonic-build precisa do `protoc` pra compilar o .proto
# em tempo de build (service/build.rs chama tonic_build::compile_protos).
# Sem isso, o cargo build quebra no build script.
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# ── Cache de dependencias ────────────────────────────────────────────────────
# Truque classico de Docker com Rust: copia so os Cargo.toml/Cargo.lock primeiro,
# compila com fontes dummy, e so depois copia o codigo real.
# Assim, enquanto as dependencias nao mudarem, o Docker reutiliza o layer cacheado
# e nao precisa baixar/compilar 200+ crates toda vez que muda uma linha de codigo.
COPY tennessee-eastman-service/Cargo.toml tennessee-eastman-service/Cargo.lock ./
COPY tennessee-eastman-service/core/Cargo.toml core/Cargo.toml
COPY tennessee-eastman-service/service/Cargo.toml service/Cargo.toml

RUN mkdir -p core/src && echo "pub fn dummy() {}" > core/src/lib.rs \
    && mkdir -p service/src && echo "fn main() {}" > service/src/main.rs

# Proto e build.rs precisam estar presentes pra resolucao de dependencias,
# porque o build.rs gera codigo a partir do .proto durante o cargo build.
COPY tennessee-eastman-service/service/proto service/proto
COPY tennessee-eastman-service/service/build.rs service/build.rs

# O || true e o 2>/dev/null sao de proposito: esse build vai falhar no link
# final (os fontes sao dummy), mas o que interessa e que todas as crates
# de dependencia ja ficam compiladas e cacheadas nesse layer.
RUN cargo build --release --bin te_service 2>/dev/null || true

# ── Build real ───────────────────────────────────────────────────────────────
COPY tennessee-eastman-service/core/src core/src
COPY tennessee-eastman-service/service/src service/src

# touch: forca o cargo a recompilar core e service (senao ele acha que o dummy
# ainda ta atualizado por causa do timestamp do layer anterior).
RUN touch core/src/lib.rs service/src/main.rs && cargo build --release --bin te_service

# ── Stage 2: runtime ─────────────────────────────────────────────────────────
# IMPORTANTE: tem que ser Trixie porque o builder (rust:latest) compila contra
# a glibc 2.41 do Trixie. Se colocar bookworm-slim aqui (glibc 2.36), o binario
# estoura com "GLIBC_2.39 not found". Ja deu esse erro, ja resolvemos.
FROM debian:trixie-slim

# ca-certificates: sem isso o tonic/hyper nao consegue fazer TLS.
# Pode nao ser necessario agora (gRPC local sem TLS), mas se um dia
# o servico precisar chamar algo externo, ja ta pronto.
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/te_service /usr/local/bin/te_service

# So esse snapshot entra na imagem — e o estado inicial da planta (baseline Exp 3).
# Os outros snapshots ficam em docs/cases/ no repo, so pra referencia.
# Nao precisa carregar peso morto no container.
COPY tennessee-eastman-service/cases/te_exp3_snapshot.toml /app/cases/te_exp3_snapshot.toml

WORKDIR /app

# 50051: porta padrao do gRPC (convencionada pelo ecossistema gRPC/protobuf)
EXPOSE 50051

# --headless: desliga o TUI (ratatui) e o CSV. So roda simulacao + gRPC.
# Esse e o modo de operacao dentro do cluster K8s.
ENTRYPOINT ["te_service", "--headless"]
