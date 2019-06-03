FROM rust

WORKDIR /app/blog/

COPY . .

RUN cargo build --release

CMD ["cargo","run","--release"]
