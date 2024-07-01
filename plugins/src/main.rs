use std::sync::mpsc::Receiver;
use anyhow::Result;

fn main() -> Result<()> {
    println!("Loading library and binding functions...");
    let lib = unsafe {
        libloading::Library::new("./libplugin1.so")?
    };
    let say_hello: libloading::Symbol<unsafe extern fn()> = unsafe {
        lib.get(b"say_hello")?
    };
    let spawn: libloading::Symbol<unsafe extern fn() -> Receiver<String>> = unsafe {
        lib.get(b"spawn")?
    };

    println!("Calling the simple function");
    unsafe { say_hello(); }

    println!("Trying the complicated case: spawn a thread and connect via a channel");
    let rx = unsafe { spawn() };
    while let Ok(message) = rx.recv() {
        println!("{message}");
    }

    Ok(())
}
