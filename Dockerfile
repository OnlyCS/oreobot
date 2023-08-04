FROM rust:latest

COPY ./ ./

RUN cargo prisma db push
RUN cargo build --release

CMD ["cargo", "run", "--release"]