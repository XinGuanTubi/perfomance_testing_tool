#========================
# Making Product Image
#========================

FROM ubuntu:latest

ENV LANG=C.UTF-8
ENV PATH="${PATH}:/opt/tubi/bin/"

WORKDIR /opt/app/perfomance_testing_tool

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y wget curl vim && \
    mkdir -p /opt/tubi/bin/ && \
    mkdir -p /opt/app/perfomance_testing_tool/

COPY target/x86_64-unknown-linux-musl/release/perfomance_testing_tool /opt/app/perfomance_testing_tool/

ENTRYPOINT ["/opt/app/perfomance_testing_tool/perfomance_testing_tool"]
