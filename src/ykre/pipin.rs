use std::{io, thread, time, sync::mpsc};
use std::sync::mpsc::Receiver;

pub fn spawn_stdin() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    let _child = thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap()
    });
    rx
}

pub fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}
