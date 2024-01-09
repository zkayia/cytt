
FROM rust:1.75-alpine as builder

RUN apk add pkgconfig openssl-dev libc-dev

WORKDIR /usr/src

RUN USER=root cargo new cytt

WORKDIR /usr/src/cytt

COPY Cargo.toml Cargo.lock .

RUN cargo build --release

RUN rm -f target/release/deps/cytt*

COPY . .

RUN cargo build --release



FROM alpine:3.19 

RUN apk add --no-cache tzdata libgcc

RUN addgroup -g 1000 cytt
RUN adduser -D -s /bin/sh -u 1000 -G cytt cytt

WORKDIR /home/cytt/bin/

RUN chown cytt:cytt .

COPY --from=builder /usr/src/cytt/target/release/cytt .
COPY assets/static/ assets/static/
COPY assets/templates/ assets/templates/

ENV TZ=Europe/Paris
ENV CYTT_IS_DOCKER=true

USER cytt
EXPOSE 8000
ENTRYPOINT ["/home/cytt/bin/cytt"]
