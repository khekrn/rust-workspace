struct Person{
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }

    fn new_instance(name: String, age: u8) -> Self {
        Self { name, age }
    }
}

fn struct_lesson_1(){
    let mut peter = Person{
        name: String::from("Peter"),
        age: 32
    };
    println!("{} is {} years old", peter.name, peter.age);
    peter.age = 33;
    println!("{} is {} years old", peter.name, peter.age);

    let jackie = Person{
        name: String::from("Jackie"),
        ..peter
    };
    println!("{} is {} years old", jackie.name, jackie.age);
}

fn struct_lesson_2(){
    let john = Person::new("John Wick".to_string(), 44);
    println!("{} is {} years old", john.name, john.age);

    let harry = Person::new_instance("Harry Potter".to_string(), 34);
    println!("{} is {} years old", harry.name, harry.age);
}

fn main(){
    struct_lesson_1();

    println!();

    struct_lesson_2();
}