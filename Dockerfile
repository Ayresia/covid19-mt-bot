FROM rust:1.59 as builder
WORKDIR /usr/src/covid19-mt-bot
COPY . .
RUN cargo install --path .
CMD ["covid19-mt-bot"]
