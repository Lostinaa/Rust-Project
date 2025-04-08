fn main() {

    let x: i32;
    let y:i32;


  let mut z =12;
   

    assert_eq!(x,5);

    println!("got em");
}


use std::sync::Mutex;
use std::thread;

let counter = Mutex::new(0);

fn increment() {
    for _ in 0..10000 {
        let mut data = counter.lock().unwrap();
        *data += 1;
    }
}

fn main() {
    let t1 = thread::spawn(increment);
    let t2 = thread::spawn(increment);

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Counter: {}", counter.lock().unwrap());
}

