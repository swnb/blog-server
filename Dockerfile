FROM rust:latest as Builder
WORKDIR /app/blog/
COPY . .
RUN ls 
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
RUN ls 
ENV  MODE product
ENV SERVER 0.0.0.0
ENV SERVER_PORT 80
EXPOSE 80
COPY --from=0 /app/blog/target/release/blog .
ENTRYPOINT [ "./blog" > "./log" ]
