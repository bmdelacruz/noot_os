mkdir -p target/x86_64-unknown-uefi/debug/esp/EFI/boot

cp \
  target/x86_64-unknown-uefi/debug/noot_os.efi \
  target/x86_64-unknown-uefi/debug/esp/EFI/boot/BootX64.efi
