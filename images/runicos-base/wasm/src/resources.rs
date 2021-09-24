use alloc::vec::Vec;
use crate::intrinsics;
use core::{
    mem::MaybeUninit,
    convert::TryInto,
    fmt::{self, Formatter, Display},
};

pub struct Resource {
    id: u32,
}

impl Resource {
    pub fn read_to_end(name: &str) -> Result<Vec<u8>, ResourceError> {
        const BLOCK_SIZE: usize = 1024;

        let mut dest: Vec<u8> = Vec::new();
        let mut resource = Resource::open(name)?;

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

                let bytes_read = resource.read(uninitialized_bytes)?;
                dest.set_len(previous_length + bytes_read);

                if bytes_read == 0 {
                    // End of file
                    break;
                }
            }
        }

        Ok(dest)
    }

    pub fn open(name: &str) -> Result<Self, ResourceError> {
        unsafe {
            let id = intrinsics::rune_resource_open(
                name.as_ptr(),
                name.len() as u32,
            );

            if id >= 0 {
                Ok(Resource {
                    id: id.try_into().unwrap(),
                })
            } else {
                Err(ResourceError::OpenFailed)
            }
        }
    }

    pub fn read(
        &mut self,
        buffer: &mut [MaybeUninit<u8>],
    ) -> Result<usize, ResourceError> {
        unsafe {
            let bytes_read = intrinsics::rune_resource_read(
                self.id,
                buffer.as_mut_ptr().cast(),
                buffer.len() as u32,
            );

            if bytes_read >= 0 {
                Ok(bytes_read as usize)
            } else {
                Err(ResourceError::ReadFailed)
            }
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ResourceError {
    OpenFailed,
    ReadFailed,
}

impl Display for ResourceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ResourceError::OpenFailed => {
                f.write_str("Unable to open the resource")
            },
            ResourceError::ReadFailed => {
                f.write_str("Unable to read from the resource")
            },
        }
    }
}
