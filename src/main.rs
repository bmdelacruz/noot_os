#![no_std]
#![no_main]
#![feature(alloc)]
#![feature(const_slice_len)]
#![feature(slice_patterns)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate alloc;

use uefi::prelude::*;

#[no_mangle]
pub extern "win64" fn uefi_start(
  image: uefi::Handle, system_table: SystemTable<Boot>
) -> Status {
  use uefi::table::boot::MemoryDescriptor;
  use uefi::table::runtime::ResetType;

  uefi_services::init(&system_table)
    .expect_success("failed to init uefi svcs");

  system_table.stdout().reset(false)
    .expect_success("failed to reset stdout");

  info!("shutting down in 3 seconds...");

  system_table.boot_services().stall(3_000_000);

  let max_mmap_size = system_table.boot_services()
    .memory_map_size() + 8 * core::mem::size_of::<MemoryDescriptor>();
  let mut mmap_storage = vec![0; max_mmap_size].into_boxed_slice();
  let (system_table, _) = system_table
    .exit_boot_services(image, &mut mmap_storage)
    .expect_success("failed to exit boot services");

  unsafe {
    system_table.runtime_services()
      .reset(ResetType::Shutdown, Status::SUCCESS, None);
  }
}