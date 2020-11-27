mod ownership;
mod lifetime;
mod lifetime_struct;
mod rc_demo;
mod arc_demo;

fn main() {
    // ownership::main();
    // lifetime::main();
    // lifetime_struct::main();
    // rc_demo::main();
    arc_demo::main();
    println!("Done!!!");
}
