qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/demo01.bin,addr=0x80200000 \
    -s -S


riscv64-unknown-elf-gdb \
    -ex 'file target/riscv64gc-unknown-none-elf/release/demo01' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'