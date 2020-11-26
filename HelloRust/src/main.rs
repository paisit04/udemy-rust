#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;
mod sh;

const MEANING_OF_LIFE:u8 = 42; // no fixed address

static mut Z:i32 = 123;

fn scope_and_shadowing()
{
    let a = 123;
    {
        let b = 456;
        println!("inside, b={}", b);

        let a = 777;
        println!("inside, a = {}", a);
    }

    println!("a = {}", a);
    //println!("outside, b={}", b);
}

fn operators()
{
    let mut a = 2+3*4;
    println!("{}", a);
    a = a+1; // not support -- ++
    a -= 2; // a = a-2
    println!("{}", a);
    println!("remainder of {} / {} = {}", a, 3, (a%3));

    let a_cubed = i32::pow(a,3);
    println!("{} cubed is of {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    println!("{} cubed is of {}", b, b_cubed);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{}^pi is of {}", b, b_to_pi);

    // bitwise
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
                        // 01 OR 10 = 11
    println!("1|2 = {}", c);

    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0;
    // > <= >= ==
    let x = 5;
    let x_is_5 = x == 5;
}

fn fundametal_data_types()
{
    let a: u8 = 123; // u = unsigned, 8 bits, 0-255
    println!("a = {}", a); // immutable

    // u = unsigned, 0 to 2^N-1
    // i = signed, -2^(N-1) .. 2^(N-1)-1
    let mut b: i8 = 0; // -128 -- 127
    println!("b = {} before", b);
    b = 42;
    println!("b = {} after", b);

    let mut c = 123456789; // i32
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));

    // usize isize
    let z: isize = 123;
    let size_of_z: usize = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z*8);

    let d: char = 'x';
    println!("{} is a char, size = {} bytes", d, mem::size_of_val(&d));

    // f32 f64 IEEE754
    let e: f32 = 2.5;
    println!("{}, size = {} bytes", e, mem::size_of_val(&e));

    let f: f64 = 2.5;
    println!("{}, size = {} bytes", f, mem::size_of_val(&f));

    let g: bool = false; //true
    println!("{}, size = {} bytes", g, mem::size_of_val(&g));
}

fn main() {
    //fundametal_data_types();
    //operators();
    //scope_and_shadowing();

    // println!("{}", MEANING_OF_LIFE);
    // unsafe {
    //     Z = 777;
    //     println!("{}", Z);
    // }

    sh::stack_and_heap();
}
