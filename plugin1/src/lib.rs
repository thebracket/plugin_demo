use std::sync::mpsc::{channel, Receiver};

#[no_mangle]
pub fn say_hello() {
    println!("Hello From Plugin1");
}

#[no_mangle]
pub fn spawn() -> Receiver<String> {
    let (tx, rx) = channel::<String>();
    std::thread::spawn(move || {
        for i in 0 .. 10 {
            tx.send(format!("Message {i} from plugin")).unwrap();
        }
    });
    rx
}