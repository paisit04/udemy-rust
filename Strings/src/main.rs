use rand::Rng;
use std::io::stdin;

fn strings()
{
    // utf-8
    let s: &'static str = "hello there!"; // &str = string slice
    //s = "abc";
    //let h = s[0];

    for c in s.chars().rev()
    {
        println!("{}", c);
    }
    println!("{:?}", s);

    if let Some(first_char) = s.chars().nth(0)
    {
        println!("first letter is {}", first_char);
    }

    // heap
    // String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8)
    {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}", letters);

    // &str <> String
    let u:&str = &letters;

    // concatenation
    // String + str
    let z = letters + "abc";
    // let z = letters + &letters;
    println!("{}", z);

    let mut abc = String::from("hello world");
    // let mut abc = "hello world".to_string();
    println!("{}", abc);
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "goodbye"));

}

fn str_format()
{
    let name = "Dmitri";
    let greeting = format!("hi, I'm {}, nice to meet you", name);
    println!("{}", greeting);

    let hello = "hello";
    let rust = "rust";
    let hello_rust = format!("{}, {}!", hello, rust);
    println!("{}", hello_rust);

    let run = "run";
    let forest = "forest";
    let rfr = format!("{0}, {1}, {0}", run, forest);
    println!("{}", rfr);

    let info = format!(
        "the name's {last}. {first} {last}.",
        first = "james",
        last = "bond"
    );
    println!("{}", info);

    let mixed = format!(
        "{1} {} {0} {} {data}",
        "alpha",
        "beta",
        data = "delta"
    );
    println!("{}", mixed);
}

fn guessing()
{
    let number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Enter your guess: ");

        let mut buffer = String::new();

        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("Your guess is out of range");
                        } else if guess < number {
                            println!("Your guess is too low");
                        } else if guess > number {
                            println!("Your guess is too high");
                        } else {
                            println!("Correct!");
                            break;
                        }
                    },
                    Err(e) => {
                        println!("Count not read your input. {}. Try again!", e);
                    }
                }
            },
            Err(_) => {
                continue;
            }
        }
    }
}

fn main() {
    // strings();
    // str_format();
    guessing();
    println!("Done!!!");
}
