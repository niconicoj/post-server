FROM rust:latest

WORKDIR /usr/src/post-server
COPY . .

RUN rustup component add rustfmt
RUN cargo install --path .

EXPOSE 50051

CMD ["post-server"]
