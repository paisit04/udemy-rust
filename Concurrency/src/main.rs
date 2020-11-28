use std::thread;
use std::time;

fn main() {
    let handle = thread::spawn(|| {
        for _ in 1..10 {
            print!("+");
            thread::sleep(time::Duration::from_micros(500));
        }
    });

    for _ in 1..10 {
        print!("-");
        thread::sleep(time::Duration::from_micros(300));
    }

    handle.join();

    println!("Done!!!");
}
