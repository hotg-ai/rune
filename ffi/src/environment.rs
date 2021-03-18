use std::{
    mem::MaybeUninit,
    os::raw::c_int,
    sync::{Arc, Mutex},
};
use anyhow::{Context, Error};
use libc::c_void;
use rune_runtime::{capability::Capability, outputs::Output};
use crate::Callbacks;

#[derive(Clone)]
pub struct Environment(Arc<Mutex<Callbacks>>);

impl Environment {
    pub fn new(cb: Callbacks) -> Self { Environment(Arc::new(Mutex::new(cb))) }

    fn get_capability<F>(
        &self,
        get_constructor: F,
    ) -> Result<Box<dyn Capability>, Error>
    where
        F: FnOnce(
            &Callbacks,
        ) -> Option<
            unsafe extern "C" fn(*mut c_void, *mut crate::Capability) -> c_int,
        >,
    {
        let cb = self.0.lock().unwrap();

        let constructor = get_constructor(&cb).context("Not Supported")?;

        unsafe {
            let mut cap = MaybeUninit::uninit();

            if constructor(cb.user_data, cap.as_mut_ptr()) == 0 {
                Ok(Box::new(cap.assume_init()))
            } else {
                Err(Error::msg("Not Supported"))
            }
        }
    }

    fn get_output<F>(
        &self,
        get_constructor: F,
    ) -> Result<Box<dyn Output>, Error>
    where
        F: FnOnce(
            &Callbacks,
        ) -> Option<
            unsafe extern "C" fn(*mut c_void, *mut crate::Output) -> c_int,
        >,
    {
        let cb = self.0.lock().unwrap();

        let constructor = get_constructor(&cb).context("Not Supported")?;

        unsafe {
            let mut out = MaybeUninit::uninit();

            if constructor(cb.user_data, out.as_mut_ptr()) == 0 {
                Ok(Box::new(out.assume_init()))
            } else {
                Err(Error::msg("Not Supported"))
            }
        }
    }
}

impl rune_runtime::Environment for Environment {
    fn log(&self, msg: &str) {
        let cb = self.0.lock().unwrap();

        if let Some(log) = cb.log {
            unsafe {
                log(cb.user_data, msg.as_ptr().cast(), msg.len() as c_int);
            }
        }
    }

    fn new_random(&self) -> Result<Box<dyn Capability>, Error> {
        self.get_capability(|cb| cb.random)
    }

    fn new_accelerometer(&self) -> Result<Box<dyn Capability>, Error> {
        self.get_capability(|cb| cb.accelerometer)
    }

    fn new_image(&self) -> Result<Box<dyn Capability>, Error> {
        self.get_capability(|cb| cb.image)
    }

    fn new_serial(&self) -> Result<Box<dyn Output>, Error> {
        self.get_output(|cb| cb.serial)
    }
}
