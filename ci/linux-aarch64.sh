set -ex

mkdir -m 777 /qemu
cd /qemu

curl -LO http://ftp.debian.org/debian/dists/testing/main/installer-arm64/20170828/images/netboot/debian-installer/arm64/linux
curl -LO http://ftp.debian.org/debian/dists/testing/main/installer-arm64/20170828/images/netboot/debian-installer/arm64/initrd.gz

mv linux kernel

mkdir init
cd init
gunzip -c ../initrd.gz | cpio -id
rm ../initrd.gz
cp /usr/aarch64-linux-gnu/lib/libgcc_s.so.1 usr/lib/
chmod a+w .
