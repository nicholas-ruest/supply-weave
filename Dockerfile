FROM node:20-bookworm AS web
WORKDIR /app/frontend
COPY frontend/package*.json ./
RUN npm ci
COPY frontend ./
RUN npm run build
FROM rust:1.99-bookworm AS rust
WORKDIR /app
COPY . .
COPY --from=web /app/frontend/dist frontend/dist
RUN cargo build --locked --release -p sw-api
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl && find /var/lib/apt/lists -depth -delete
WORKDIR /app
COPY --from=rust /app/target/release/sw-api /usr/local/bin/sw-api
COPY --from=web /app/frontend/dist frontend/dist
ENV PORT=8080 SUPPLY_WEAVE_DATA=/tmp/supply-weave
EXPOSE 8080
HEALTHCHECK CMD ["curl","-fsS","http://127.0.0.1:8080/health"]
ENTRYPOINT ["sw-api"]
