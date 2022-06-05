FROM rust:1.61-slim-buster

WORKDIR /usr/src/app
COPY Cargo.toml .

COPY ./src/cert.pem /tmp/cert.pem
COPY ./src/key.pem /tmp/key.pem
EXPOSE 4433/udp
#RUN cargo install --path .
CMD ["/bin/sh"] 


