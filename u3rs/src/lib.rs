extern crate u3_alloc;

#[no_mangle]
pub extern fn hello(x: usize) {
    println!("Hello, {}", x);
}
