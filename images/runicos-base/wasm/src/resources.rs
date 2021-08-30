use alloc::vec::Vec;
use crate::intrinsics;
use core::mem::MaybeUninit;

pub struct Resource {
    id: u32,
}

impl Resource {
    pub fn read_to_end(name: &str) -> Vec<u8> {
        const BLOCK_SIZE: usize = 1024;

        let mut dest: Vec<u8> = Vec::new();
        let mut resource = Resource::open(name);

        loop {
            dest.reserve(BLOCK_SIZE);
            let previous_length = dest.len();
            let capacity = dest.capacity();

            unsafe {
                // TODO: replace with dest.spare_capacity_mut() when it is
                // stable
                let uninitialized_bytes = core::slice::from_raw_parts_mut(
                    dest.as_mut_ptr().add(previous_length)
                        as *mut MaybeUninit<u8>,
                    capacity - previous_length,
                );

                let bytes_read = resource.read(uninitialized_bytes);
                dest.set_len(previous_length + bytes_read);

                if bytes_read == 0 {
                    // End of file
                    break;
                }
            }
        }

        dest
    }

    pub fn open(name: &str) -> Self {
        unsafe {
            let id = intrinsics::rune_resource_open(
                name.as_ptr(),
                name.len() as u32,
            );

            Resource { id }
        }
    }

    pub fn read(&mut self, buffer: &mut [MaybeUninit<u8>]) -> usize {
        unsafe {
            intrinsics::rune_resource_read(
                self.id,
                buffer.as_mut_ptr().cast(),
                buffer.len() as u32,
            ) as usize
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {}
}
