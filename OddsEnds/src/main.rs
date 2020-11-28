extern crate rand;
extern crate phrases;

use rand::Rng;
use phrases::greetings::french;

fn main() {
    let mut rng = rand::thread_rng();
    let b:bool = rng.gen();

    println!("English: {}, {}",
        phrases::greetings::english::hello(),
             phrases::greetings::english::goodbye(),
    );

    println!("French: {}, {}",
             french::hello(),
             french::goodbye(),
    );
    println!("Hello, world!");
}
