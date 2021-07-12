FROM lukemathwalker/cargo-chef as planner
WORKDIR bot
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef as cacher
WORKDIR bot
COPY --from=planner /bot/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as builder
WORKDIR bot
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /bot/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --release --bin bot

FROM rust as runtime
WORKDIR bot
COPY --from=builder /bot/target/release/bot /usr/local/bin
ENTRYPOINT ["/usr/local/bin/bot"]
