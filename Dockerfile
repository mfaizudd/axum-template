FROM rust AS chef
WORKDIR /app
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim AS runtime
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# TODO: Change app name
COPY --from=builder /app/target/release/axum-template /usr/local/bin/axum-template
COPY ./migrations ./migrations
COPY ./configuration ./configuration
VOLUME /app/configuration
CMD ["app"]
