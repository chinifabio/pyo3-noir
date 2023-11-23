FROM rust:slim-buster AS builder
# o slim-buster
# Poi ti fai
RUN apt-get update && apt-get install pkg-config openssl libssl-dev clang python3 python3-pip -y 
# Eventuali altri pacchetti che ti servono
RUN python3 -m pip install maturin

WORKDIR /build

### PREBUILD AND CACHE DEPENDENCIES
COPY Cargo.toml Cargo.lock ./
# Qua copi solo questi così puoi prebuildare le deps, ma se cambi qualcosa che non è cargo.toml|lock non ti invalida il layer della cache di docker (ogni comando docker crea un layer di cache)

# touch serve perché non builda se non c'è un lib.rs
RUN mkdir -p src
RUN touch src/lib.rs && cargo build --release --lib
### PREBUILD AND CACHE DEPENDENCIES

# Copi effettivamente tutto
COPY . .

RUN cargo build --release
# A questo punto per fare un'immagine da lanciare puoi fare una cosa così, che ti lascia con un'immagine piccola, altrimenti fai il discorso che ho detto prima facendo il volume mount

RUN maturin build --release

WORKDIR /build/target/wheels
RUN mkdir -p /app/ && mv noir-0.1.2-cp37-abi3-manylinux_2_34_x86_64.whl /app/ 

# RUNNER
FROM debian:bookworm-slim

COPY --from=builder /app/ /app/

WORKDIR /app
RUN mkdir venv
RUN apt-get update && apt-get install python3 python3-pip python3.11-venv -y
RUN python3 -m venv ./venv/
RUN ./venv/bin/pip3 install ./noir-0.1.2-cp37-abi3-manylinux_2_34_x86_64.whl

ENV PATH="/app:${PATH}"