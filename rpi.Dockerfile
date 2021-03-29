# Use intermediate vendor crate to avoid pulling dependencies every time
FROM --platform=$BUILDPLATFORM rust:1-slim-buster as vendor
ENV USER=root

WORKDIR /code
RUN cargo init
COPY ./Cargo.toml /code/Cargo.toml
RUN mkdir -p .cargo \ 
	&& cargo vendor > .cargo/config.toml

FROM --platform=$BUILDPLATFORM rust:1-slim-buster as builder
ENV USER=root

WORKDIR /code
RUN rustup target add armv7-unknown-linux-musleabihf
RUN apt-get -yqq update 
RUN apt-get -yqq install \
	ca-certificates \
	binutils-arm-linux-gnueabihf \
	gcc-arm-linux-gnueabihf

COPY ./Cargo.toml /code/Cargo.toml
COPY ./src /code/src
COPY --from=vendor /code/.cargo/ /code/.cargo
COPY --from=vendor /code/vendor /code/vendor

# Have to set compiler via ENV variable b/c cc does not pick settings from .cargo/config
ENV CC=arm-linux-gnueabihf-gcc
ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER=arm-linux-gnueabihf-ld

RUN cargo build \
	--release \
	--offline \
	--target armv7-unknown-linux-musleabihf

FROM debian:buster-slim
RUN apt-get -y update
RUN apt-get -y install cron

COPY --from=builder /code/target/armv7-unknown-linux-musleabihf/release/bootstrap /bootstrap
COPY --from=builder /etc/ssl/certs /etc/ssl/certs
COPY crontab.sh /crontab.sh 

RUN chmod 0744 /crontab.sh 

CMD /crontab.sh
