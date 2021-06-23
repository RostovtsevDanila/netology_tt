FROM rust:slim

WORKDIR /app

RUN apt-get update &&\
    apt-get install pkg-config -y &&\
    apt-get install libssl-dev -y

COPY ./src ./src
COPY ./Cargo.toml .
#COPY ./Cargo.lock .

RUN cargo build -j $(nproc) --release

RUN cp ./target/release/netology_tt .
RUN rm -rf ./target
RUN rm -rf ./src
RUN rm  Cargo.toml
RUN rm  Cargo.lock
RUN apt-get remove pkg-config -y
RUN apt-get remove libssl-dev -y

ENTRYPOINT [ "./netology_tt" ]
