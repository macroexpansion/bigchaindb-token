FROM rust:1.76 as builder

WORKDIR /usr/src/app

COPY Cargo.toml ./
RUN mkdir src/ && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && cargo build --release

COPY . ./
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 libpq5 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/bigchaindb-token /usr/local/bin/bigchaindb-token

EXPOSE 3000

ENV ARGS ""

CMD ["/bin/bash", "-c", "bigchaindb-token ${ARGS}"]
