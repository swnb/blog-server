FROM rust

WORKDIR /app/blog/
COPY . .
ENV SERVER_PORT 80
EXPOSE 80
RUN cargo build --release

ENTRYPOINT [ "./target/release/blog" ]
