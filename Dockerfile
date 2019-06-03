FROM rust

WORKDIR /app/blog/
COPY . .
EXPOSE 80
RUN cargo build --release

CMD ["cargo","run","--release"]
