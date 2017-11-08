FROM ubuntu:17.10

RUN apt-get update -y && apt-get install -y --no-install-recommends \
      bc \
      bzip2 \
      ca-certificates \
      cmake \
      cpio \
      curl \
      file \
      g++ \
      gcc-aarch64-linux-gnu \
      git \
      libc6-dev \
      libc6-dev-arm64-cross \
      make \
      python2.7 \
      qemu-system-aarch64 \
      xz-utils

COPY linux-aarch64.sh /
RUN bash /linux-aarch64.sh

COPY test-runner-linux /

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER="/test-runner-linux " \
    OBJDUMP=aarch64-linux-gnu-objdump