FROM debian:buster-slim as builder
WORKDIR /tmp/code
ENV RUSTUP_HOME=/usr/local/rustup \
	CARGO_HOME=/usr/local/cargo \
	PATH=/usr/local/cargo/bin:$PATH

RUN apt-get -yqq update
RUN apt-get -yqq install build-essential curl pkg-config libssl-dev ca-certificates
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh 
RUN sh rustup.sh -y --no-modify-path --profile=minimal

COPY ./src/ ./src/
COPY ./Cargo.* ./
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get -y update
RUN apt-get -y install cron

COPY --from=builder /tmp/code/target/release/bootstrap /bootstrap
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY crontab.sh /tmp/crontab.sh 

RUN chmod 0744 /tmp/crontab.sh 

ENTRYPOINT ["/tmp/crontab.sh"]
