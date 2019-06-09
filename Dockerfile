FROM rust

WORKDIR /app/blog/
COPY . .
ENV SERVER_PORT 80
ENV RUST_LOG actix_web=info
EXPOSE 80
RUN cargo build --release

ENTRYPOINT [ "./target/release/blog" ]
