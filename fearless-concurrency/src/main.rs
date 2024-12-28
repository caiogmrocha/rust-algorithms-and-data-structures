use std::thread;
use std::time;
use std::sync;

const LIMIT: i32 = 10;

fn main() {
    let (tx1, rx) = sync::mpsc::channel::<String>();

    let tx2 = tx1.clone();
    
    thread::spawn(move || {
        for i in 0..LIMIT {
            tx1.send(i.to_string()).unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for i in 0..LIMIT {
            tx2.send(i.to_string()).unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }
    });
    
    for value in rx {
        println!("{}", value);
    }
}
