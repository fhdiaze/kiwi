FROM rust AS build

RUN USER=root cargo new --bin kiwi
WORKDIR /kiwi

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/kiwi*
RUN cargo build --release

FROM rust
COPY --from=build /kiwi/target/release/kiwi .

CMD ["./kiwi"]