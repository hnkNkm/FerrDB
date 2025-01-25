FROM rust:latest

RUN apt-get update && apt-get install -y git

WORKDIR /usr/src/app

CMD ["/bin/bash"]
