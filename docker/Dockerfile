FROM rust:slim

WORKDIR /app

RUN apt-get update &&\
    apt-get install pkg-config -y &&\
    apt-get install libssl-dev -y

COPY ./src ./src
COPY ./Cargo.toml .

RUN cargo build -j $(nproc) --release

RUN cp ./target/release/netology_tt . &&\
    rm -rf ./target &&\
    rm -rf ./src &&\
    rm  Cargo.toml &&\
    rm  Cargo.lock &&\
    apt-get remove pkg-config -y &&\
    apt-get remove libssl-dev -y

ENTRYPOINT [ "./netology_tt" ]
