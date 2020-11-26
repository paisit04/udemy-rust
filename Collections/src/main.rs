use std::collections::HashMap;
use std::collections::HashSet;

fn vectors()
{
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);

    a.push(44);
    println!("a = {:?}", a);

    // usize isize

    let idx:usize = 0;

    a[idx] = 312;
    println!("a[0] = {}", a[idx]);

    match a.get(6)
    {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error, no such element")
    }

    for x in &a { println!("{}", x)};
    a.push(77);
    println!("{:?}", a);

    let last_elem = a.pop(); // Option
    println!("last elem is {:?}, a = {:?}", last_elem, a);

    while let Some(x) = a.pop()
    {
        println!("{}", x);
    }
}

fn hashmaps()
{
    let  mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} side", shapes["square".into()]);

    for (key, value) in &shapes {
        println!("{} : {}", key, value);
    }

    shapes.insert("square".into(), 5);
    println!("{:?}", shapes);

    shapes.entry("circle".into()).or_insert(1);
    {
        let actual = shapes.entry("circle".into()).or_insert(2);
        *actual = 0;
    }

    println!("{:?}", shapes);
}


fn hashsets()
{
    let mut greeks = HashSet::new();

    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}", greeks);

    greeks.insert("delta");
    println!("{:?}", greeks);

    let added_vega = greeks.insert("vega");
    if added_vega {
        println!("we added vega! hooray!");
    }

    if !greeks.contains("kappa") {
        println!("we don't have kappa");
    }

    let removed = greeks.remove("delta");
    if removed {
        println!("we removed delta");
    }
    println!("{:?}", greeks);

    let _1_5:HashSet<_> = (1..=5).collect();
    let _6_10:HashSet<_> = (6..=10).collect();
    let _1_10:HashSet<_> = (1..=10).collect();
    let _2_8:HashSet<_> = (2..=8).collect();

    // subset
    println!(
      "is {:?} a subset of {:?}? {}",
        _2_8, _1_10,
        _2_8.is_subset(&_1_10)
    );

    // disjoint = no common elements
    println!(
        "is {:?} disjoint {:?}? {}",
        _1_5, _6_10,
        _1_5.is_disjoint(&_6_10)
    );

    // union, intersection
    println!(
        "{:?} unions {:?} = {:?}",
        _2_8, _6_10,
        _2_8.union(&_6_10)
    );

    println!(
        "{:?} intersection {:?} = {:?}",
        _2_8, _6_10,
        _2_8.intersection(&_6_10)
    );

    // difference
    println!(
        "{:?} difference {:?} = {:?}",
        _2_8, _6_10,
        _2_8.difference(&_6_10)
    );
}

fn iterators()
{
    let vec = vec![3,2,1];

    for x in &vec
    {
        println!("{}", *x);
    }
    println!("{:?}", vec);

    for x in vec.iter()
    {
        println!("we got {}", x);
    }
    println!("{:?}", vec);

    let mut vec_mut = vec![3,2,1];
    for x in vec_mut.iter_mut()
    {
        *x +=2;
    }
    println!("{:?}", vec_mut);

    for x in vec_mut.iter().rev()
    {
        println!("{}", x);
    }

    let mut vec2 = vec![1, 2, 3];
    // let it = vec.into_iter();
    vec2.extend(vec);
    println!("{:?}", vec2);
    // println!("{:?}", vec); //borrow of moved value: `vec`
}

fn main() {
    // vectors();
    // hashmaps();
    // hashsets();
    iterators();
    println!("Done!!!");
}
