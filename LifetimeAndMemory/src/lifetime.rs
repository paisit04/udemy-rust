struct Person
{
    name: String
}

impl Person
{
    fn get_ref_name(&self) -> &String
    // fn get_ref_name<'a>(&'a self) -> &'a String
    {
        &self.name
    }
}

struct Company<'z>
{
    name: String,
    ceo: &'z Person
}

pub fn main()
{
    let boss = Person {name: String::from("Elon Musk")};
    let tesla = Company {name: String::from("Tesla"), ceo: &boss};

    let mut z: &String;
    {
        let p = Person {name: String::from("John")};
        z = p.get_ref_name()
    }
}