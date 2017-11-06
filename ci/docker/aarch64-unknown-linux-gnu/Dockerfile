FROM ubuntu:17.10
RUN apt-get update && apt-get install -y --no-install-recommends \
  gcc \
  curl \
  ca-certificates \
  libc6-dev \
  gcc-aarch64-linux-gnu \
  libc6-dev-arm64-cross \
  qemu-system-aarch64 \
  cpio \
  make \
  file

COPY linux-aarch64.sh /
RUN bash /linux-aarch64.sh

COPY test-runner-linux /

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER="/test-runner-linux aarch64" \
    PATH=$PATH:/rust/bin \
    OBJDUMP=aarch64-linux-gnu-objdump