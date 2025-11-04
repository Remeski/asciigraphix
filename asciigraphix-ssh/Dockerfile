FROM rust:1.91 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path asciigraphix-ssh

FROM debian:stable-slim
COPY --from=builder /usr/local/cargo/bin/asciigraphix-ssh /usr/local/bin/asciigraphix-ssh
CMD ["asciigraphix-ssh"]
