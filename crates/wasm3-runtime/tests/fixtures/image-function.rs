#[no_mangle]
pub unsafe extern "C" fn _manifest() -> i32 {
    tick();

    0
}

#[no_mangle]
pub extern "C" fn _call(_: i32, _: i32, _: i32) {}

extern "C" {
    fn tick();
}
