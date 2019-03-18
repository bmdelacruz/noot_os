qemu-system-x86_64 \
  -drive if=pflash,format=raw,file=ovmf/OVMF_CODE.fd,readonly=on \
  -drive if=pflash,format=raw,file=ovmf/OVMF_VARS.fd,readonly=on \
  -drive format=raw,file=fat:rw:target/x86_64-unknown-uefi/debug/esp
