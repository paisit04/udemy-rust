use std::fmt::Debug;
trait Shape
{
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Circle
{
    radius: f64
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}


//fn print_info(shape: impl Shape + Debug)
// fn print_info<T: Shape + Debug>(shape: T)
fn print_info<T>(shape: T)
    where T: Shape + Debug
{
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}

pub fn trait_params_fn()
{
    let c = Circle{ radius: 2.0 };
    print_info(c);
}