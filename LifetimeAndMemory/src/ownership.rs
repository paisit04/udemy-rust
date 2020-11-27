pub fn main()
{
    let print_vector = |x:&Vec<i32>|
        {
            println!("{:?}",x);
        };

    let v = vec![1,2,3];
    print_vector(&v);
    println!("{}", v[0]);

    let mut a = 40;
    {
        let b = &mut a;
        *b += 2;
    }
    println!("a = {}", a);

    let mut z = vec![3,2,1];
    for i in &z
    {
        println!("i = {}", i);
    }
}