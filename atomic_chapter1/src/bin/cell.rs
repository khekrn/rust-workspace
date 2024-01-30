use std::cell::Cell;

#[derive(Copy, Clone, Debug)]
struct Person<'a>{
    name: &'a str,
    age: i32
}

impl <'a> Person <'a>{
    fn new(name: &'a str, age: i32) -> Person<'a>{
        return Person{name, age }
    }
}


fn main() {
    let kk = Person::new("Kishore Karunakaran", 32);
    let asw = Person::new("Aswin Karunakaran", 27);

    let c1 = Cell::new(kk);
    let c2 = Cell::new(asw);

    println!("Before Updating \n");
    println!("C1 = {:?}", c1);
    println!("C2 = {:?}", c2);


    let kk_temp = c1.as_ptr();
    let as_temp = c2.as_ptr();

    unsafe {
        (*kk_temp).age = 33;
        (*kk_temp).name = "K Kishore";

        (*as_temp).age = 28;
        (*as_temp).name = "K Aswin";
    }

    println!("After Updating \n");
    println!("C1 = {:?}", c1);
    println!("C2 = {:?}", c2);

    c1.set(kk);
    c2.set(asw);
    println!("Resetting to default \n");
    println!("C1 = {:?}", c1);
    println!("C2 = {:?}", c2);
}