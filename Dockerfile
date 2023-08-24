FROM alpine:latest

COPY ./target/release/oreo-bot ./oreo-bot
COPY ./.env ./.env

CMD ["./oreo-bot"]