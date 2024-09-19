FROM rust:1.74.0 as builder

RUN apt-get update && apt-get install -y \
    mingw-w64 \
    gcc-mingw-w64-x86-64 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app


COPY . .
RUN rustup target add x86_64-pc-windows-gnu

RUN cargo build --release --target x86_64-pc-windows-gnu

FROM rust:alpine3.17

COPY --from=builder /usr/src/app/target/x86_64-pc-windows-gnu/release/rusty-chat.exe /usr/local/bin/rusty-chat.exe

EXPOSE 8080

# CMD ["/usr/local/bin/rusty-chat"]