FROM rust:1.61 as builder 



COPY ./src/ /app/src/
COPY ./Cargo.lock /app/Cargo.lock
COPY ./Cargo.toml /app/Cargo.toml
COPY ./textures /app/textures
WORKDIR /app
RUN cargo build --release
RUN apt-get update && apt-get install -y bash


FROM debian:buster-slim
RUN apt-get update -y && apt-get install sudo -yq \
    && sudo apt-get install libx11-6 -yq \
    && apt-get install openssl -yq \
    && apt-get install libssl-dev -yq \
    && sudo apt-get install libxcursor-dev -yq \
    && sudo apt-get install libxrandr-dev -yq \
    && sudo apt-get install libxi-dev -yq \
    && sudo apt-get install libx11-dev -yq \
    && sudo apt-get install libx11-xcb-dev -yq \
    && rm -rf /var/lib/apt/lists/* 
COPY --from=builder /app/textures/ app/textures/
COPY --from=builder /app/target/release /app/target/release

CMD ["/app/target/release/dug", "--no-GUI"]