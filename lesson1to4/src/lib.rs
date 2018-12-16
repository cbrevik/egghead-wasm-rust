extern "C" {
    fn appendNumberToBody(x: u32);
    fn alert(x: u32);
}

#[no_mangle]
pub extern "C" fn run() {
    unsafe {
        appendNumberToBody(42);
        alert(4);
    }
}
