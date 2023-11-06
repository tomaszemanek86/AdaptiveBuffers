FROM ubuntu:23.10

RUN echo "root:AB" | chpasswd
RUN useradd --create-home --shell=/bin/bash AB
RUN echo "AB:AB" | chpasswd
RUN usermod -aG sudo AB
RUN echo "AB ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers

RUN apt-get update

RUN apt-get install -y \
    build-essential \
    curl \
    git

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH=/root/.cargo/bin:$PATH

CMD ["sleep", "infinity"]