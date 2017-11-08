set -ex

mkdir -m 777 /qemu
cd /qemu

#curl -LO https://github.com/qemu/qemu/raw/master/pc-bios/s390-ccw.img
curl -LO http://ftp.debian.org/debian/dists/testing/main/installer-arm64/20170828/images/generic/kernel.debian
curl -LO http://ftp.debian.org/debian/dists/testing/main/installer-arm64/20170828/images/generic/initrd.debian

mv kernel.debian kernel
mv initrd.debian initrd.gz

mkdir init
cd init
gunzip -c ../initrd.gz | cpio -id
rm ../initrd.gz
ls /usr/
cp /usr/arm64-linux-gnu/lib/libgcc_s.so.1 usr/lib/
chmod a+w .
