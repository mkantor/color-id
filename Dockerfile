FROM rust:1.26
WORKDIR /opt/color-id

# First, just get all the dependencies. This is the slowest step, so it's nice
# to have this layer cached separately to speed up subsequent builds.
COPY Cargo.* ./
RUN mkdir -p src && touch src/main.rs && cargo fetch

COPY src/* src/
RUN cargo install

ENTRYPOINT ["color-id"]
