FROM rust:1-alpine

WORKDIR /usr/src/app

COPY src ./src

COPY Cargo.toml .

RUN cargo install --path .

CMD ["calcanalyzer"]