use std::{
    ffi::c_void,
    mem::{ManuallyDrop, MaybeUninit},
    os::raw::{c_char, c_int},
    sync::Arc,
};
use anyhow::Error;
use runicos_base::BaseImage;
use crate::{Output, capability::Capability};

/// A vtable providing the Rune with functions for interacting with its
/// environment.
///
/// # Safety
///
/// All methods must be thread-safe and manage its own synchronisation.
///
/// The `user_data` object must also be safe to move between threads.
#[repr(C)]
pub struct Callbacks {
    /// Arbitrary user data that can be used to persist state between method
    /// calls.
    pub user_data: *mut c_void,
    /// Log a message with the specified level, module name, and message.
    pub log: Option<
        unsafe extern "C" fn(
            *mut c_void,
            c_int,
            *const c_char,
            c_int,
            *const c_char,
            c_int,
        ),
    >,
    /// Initialize a capability vtable which produces random data.
    pub random:
        Option<unsafe extern "C" fn(*mut c_void, *mut Capability) -> c_int>,
    /// Initialize a capability vtable which yields accelerometer samples.
    ///
    /// Each sample consists of three 32-bit floating point values, with the
    /// integer argument specifying how many samples the buffer can hold.
    pub accelerometer:
        Option<unsafe extern "C" fn(*mut c_void, *mut Capability) -> c_int>,
    /// Initialize a capability vtable which yields images.
    pub image:
        Option<unsafe extern "C" fn(*mut c_void, *mut Capability) -> c_int>,
    /// Initialize a capability vtable which yields 16-bit PCM audio.
    pub sound:
        Option<unsafe extern "C" fn(*mut c_void, *mut Capability) -> c_int>,
    /// Initialize the serial output vtable.
    pub serial: Option<unsafe extern "C" fn(*mut c_void, *mut Output) -> c_int>,
    /// A destructor for the cleaning up the user data when the module is
    /// unloaded.
    pub destroy: Option<unsafe extern "C" fn(*mut c_void)>,
}

impl Drop for Callbacks {
    fn drop(&mut self) {
        if let Some(destroy) = self.destroy {
            unsafe {
                destroy(self.user_data);
            }
        }
    }
}

// Safety: Upheld by the caller.
unsafe impl Send for Callbacks {}
unsafe impl Sync for Callbacks {}

impl rune_runtime::Image for Callbacks {
    fn initialize_imports(self, registrar: &mut dyn rune_runtime::Registrar) {
        // Use ManuallyDrop to avoid accidentally calling the destructor
        let callbacks = ManuallyDrop::new(self);

        let Callbacks {
            user_data,
            log,
            random,
            accelerometer,
            image,
            sound,
            serial,
            destroy,
        } = &*callbacks;
        let user_data = Arc::new(UserData {
            data: *user_data,
            destroy: *destroy,
        });

        // Instead of implementing *everything* ourselves, we can just use the
        // normal base image and inject the native library's implementation.

        let mut base_image = BaseImage::default();

        if let Some(accelerometer) = *accelerometer {
            base_image.with_accelerometer(cap(&user_data, accelerometer));
        }
        if let Some(random) = *random {
            base_image.with_rand(cap(&user_data, random));
        }
        if let Some(img) = *image {
            base_image.with_image(cap(&user_data, img));
        }
        if let Some(sound) = *sound {
            base_image.with_sound(cap(&user_data, sound));
        }

        if let Some(serial) = *serial {
            base_image.with_serial(out(&user_data, serial));
        }

        if let Some(log) = *log {
            base_image.with_log(log_func(&user_data, log));
        }

        base_image.initialize_imports(registrar);
    }
}

fn log_func(
    user_data: &Arc<UserData>,
    func: unsafe extern "C" fn(
        *mut c_void,
        c_int,
        *const c_char,
        c_int,
        *const c_char,
        c_int,
    ),
) -> impl Fn(&log::Record) -> Result<(), Error> + Sync + Send + 'static {
    let user_data = Arc::clone(user_data);

    move |record| {
        let level = record.level();
        let target = record.target();
        let message = format!("{}", record.args());

        unsafe {
            func(
                user_data.data,
                level as i32,
                target.as_ptr().cast(),
                target.len() as c_int,
                message.as_ptr().cast(),
                message.len() as c_int,
            );
        }

        Ok(())
    }
}

fn cap(
    user_data: &Arc<UserData>,
    func: unsafe extern "C" fn(*mut c_void, *mut Capability) -> c_int,
) -> impl Fn() -> Result<Box<dyn rune_runtime::Capability>, Error>
       + Send
       + Sync
       + 'static {
    let user_data = Arc::clone(user_data);

    move || {
        let mut capability = MaybeUninit::uninit();

        unsafe {
            let ret = func(user_data.data, capability.as_mut_ptr());

            if ret == 0 {
                Ok(Box::new(capability.assume_init()))
            } else {
                anyhow::bail!("Initializing the capability returned non-zero exit code: {}", ret)
            }
        }
    }
}

fn out(
    user_data: &Arc<UserData>,
    func: unsafe extern "C" fn(*mut c_void, *mut Output) -> c_int,
) -> impl Fn() -> Result<Box<dyn rune_runtime::Output>, Error> + Send + Sync + 'static
{
    let user_data = Arc::clone(user_data);

    move || {
        let mut output = MaybeUninit::uninit();

        unsafe {
            let ret = func(user_data.data, output.as_mut_ptr());

            if ret == 0 {
                Ok(Box::new(output.assume_init()))
            } else {
                anyhow::bail!(
                    "Initializing the output returned non-zero exit code: {}",
                    ret
                )
            }
        }
    }
}

/// A wrapper around some opaque object which makes sure it gets free'd.
struct UserData {
    data: *mut c_void,
    destroy: Option<unsafe extern "C" fn(*mut c_void)>,
}

impl Drop for UserData {
    fn drop(&mut self) {
        if let Some(destroy) = self.destroy {
            unsafe {
                destroy(self.data);
            }
        }
    }
}

// Safety: Same safety invariants as Callbacks (all synchronisation upheld by
// caller).
unsafe impl Send for UserData {}
unsafe impl Sync for UserData {}
