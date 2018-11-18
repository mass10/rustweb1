FROM ubuntu:latest

MAINTAINER mass10

ENV DEBIAN_FRONTEND noninteractive

RUN apt-get update
RUN apt-get install curl -y
RUN mkdir /tmp/rustup
COPY rustup /tmp/rustup/
RUN /tmp/rustup/rustup -y
# RUN curl https://sh.rustup.rs -sSf | sh -s -y -- --help

COPY . /myapp

WORKDIR /myapp

ENV PATH ~/.cargo/bin:$PATH

CMD find / -type f -name cargo
CMD ~/.cargo/bin/cargo run
