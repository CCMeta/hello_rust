#!/bin/sh
fastboot flash ram ".\u-boot-with-spl.bin"
fastboot reboot
fastboot flash uboot ".\u-boot-with-spl.bin"

env set ipaddr 192.168.1.202

dhcp 0x40000000 192.168.1.201:Image+++

0x000000003fb39000

env set findpart "rollback; if test ${boot_partition} = bootB; then mmcbootpart=4; else mmcbootpart=2; fi; if test ${root_partition} = rootfsB; then mmcpart=5; else mmcpart=3; fi;"
# initrd=0x32000000,0xa00000 root=/dev/ram0 console=ttySAC0 mem=64M init=/linuxrc

env set ipaddr 192.168.0.8
env set serverip 192.168.10.125

tftp 0xffffef8000 light_aon_fpga.bin
tftp 0xffc0000000 light_c906_audio.bin
# very important !!!!! if not jam of [starting kernel...]
tftp 0x00000000 fw_dynamic.bin
# not must be 0xc0000000 any empty address will be OK
tftp 0xc0000000 vmlinux-5.10.113-lpi4a
tftp 0x01f00000 light-lpi4a.dtb
tftp 0xc0000000 os.bin


booti 0xc0000000 - 0x01f00000

sysboot mmc 0:2 any 0xd0000000 /extlinux/extlinux.conf


./mkimage -A riscv -O linux -T kernel -C gzip -a 0x00200000 -e 0x00200000 -d ./Image.gz uImage
