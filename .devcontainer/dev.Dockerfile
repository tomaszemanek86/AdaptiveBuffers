FROM ubuntu:23.10

RUN apt-get update

RUN apt-get install -y build-essential curl git

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH=/root/.cargo/bin:$PATH

RUN apt-get install -y  cmake gcc g++ valgrind

CMD ["sleep", "infinity"]