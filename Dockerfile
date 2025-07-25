FROM rust:trixie AS chef
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall cargo-chef -y
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

# Install `dx`
RUN cargo binstall dioxus-cli@0.7.0-alpha.2 --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"

# Create the final bundle folder. Bundle always executes in release mode with optimizations enabled
RUN dx bundle --platform web --release

FROM rust:slim-trixie AS runtime
WORKDIR /usr/local/app
COPY --from=builder /app/target/dx/ed-li/release/web/ .

# set our port and make sure to listen for all connections
ENV PORT=8080
ENV IP=0.0.0.0

# expose the port 8080
EXPOSE 8080

ENTRYPOINT [ "./ed-li" ]
