FROM rust:1.81.0-slim-bookworm AS build
WORKDIR /game
COPY game .

# Needed to compile into static binary
# https://stackoverflow.com/questions/31770604/how-to-generate-statically-linked-executables/31778003#31778003
RUN rustup target add x86_64-unknown-linux-musl && \
	cargo build --target=x86_64-unknown-linux-musl --release

FROM alpine:3.20
WORKDIR /game
COPY --from=build /game/target/x86_64-unknown-linux-musl/release/terminal_rpg .
CMD ["./terminal_rpg"]
