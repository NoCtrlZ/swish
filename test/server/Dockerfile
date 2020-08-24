FROM rust:1.40

WORKDIR /app

COPY . .

RUN cargo build

EXPOSE 3000

ENTRYPOINT [ "./target/debug/test" ]
