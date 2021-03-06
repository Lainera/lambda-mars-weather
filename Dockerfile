# Use intermediate vendor crate to avoid pulling dependencies every time
FROM --platform=$BUILDPLATFORM rust:1-slim-buster as vendor
ENV USER=root

WORKDIR /code
RUN cargo init
COPY ./Cargo.toml /code/Cargo.toml
RUN mkdir -p .cargo \ 
	&& cargo vendor > .cargo/config.toml

FROM rust:1-slim-buster as builder
ENV USER=root

WORKDIR /code
RUN apt-get -yqq update 
RUN apt-get -yqq install ca-certificates 

COPY ./Cargo.toml /code/Cargo.toml
COPY ./src /code/src
COPY --from=vendor /code/.cargo/ /code/.cargo
COPY --from=vendor /code/vendor /code/vendor

RUN cargo build \
	--release \
	--offline 

FROM debian:buster-slim
RUN apt-get -y update
RUN apt-get -y install cron

COPY --from=builder /code/target/release/bootstrap /bootstrap
COPY --from=builder /etc/ssl/certs /etc/ssl/certs
COPY crontab.sh /crontab.sh 

RUN chmod 0744 /crontab.sh 

CMD /crontab.sh
