#!/bin/sh

set -ex

: ${TARGET?"The TARGET environment variable must be set."}
: ${CUDA?"The CUDA environment variable must be set."}

# Install ptx-builder dependencies:
cargo install xargo
cargo install ptx-linker
rustup component add rust-src

# Install CUDA:
wget http://developer.download.nvidia.com/compute/cuda/repos/ubuntu1404/x86_64/cuda-repo-ubuntu1404_${CUDA}_amd64.deb
sudo dpkg -i cuda-repo-ubuntu1404_${CUDA}_amd64.deb
sudo apt-get update -qq
export CUDA_APT=${CUDA:0:3}
export CUDA_APT=${CUDA_APT/./-}
sudo apt-get install -y cuda-drivers cuda-core-${CUDA_APT} cuda-cudart-dev-${CUDA_APT}
sudo apt-get clean
export CUDA_HOME=/usr/local/cuda-${CUDA:0:3}
export LD_LIBRARY_PATH=${CUDA_HOME}/nvvm/lib64:${LD_LIBRARY_PATH}
export LD_LIBRARY_PATH=${CUDA_HOME}/lib64:${LD_LIBRARY_PATH}
export PATH=${CUDA_HOME}/bin:${PATH}

cargo build crates/nvptx-test
