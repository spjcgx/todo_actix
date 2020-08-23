FROM debian:bullseye-slim
WORKDIR /app
ADD target/release/todo_actix .
CMD ["/app/todo_actix"]