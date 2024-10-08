use std::{sync::{Mutex, Arc}, thread};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10{
        // counter variable shadowed in this scope
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move ||{
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    // Both below lines do the same thing because of automatic referencing and dereferencing rules:
    // println!("Result: {}", *((*counter).lock().unwrap()));
    println!("Result: {}", *counter.lock().unwrap());
}
