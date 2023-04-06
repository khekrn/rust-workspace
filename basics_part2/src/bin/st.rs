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

#[derive(Debug)]
struct Race{
    name: String,
    laps: Vec<i32>,
}

impl Race {
    fn new(name: &str)-> Race {
        Race { name: String::from(name), laps: Vec::new() }
    }

    fn add_lap(&mut self, lap: i32){
        self.laps.push(lap);
    }

    fn print_laps(&self){
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate(){
            println!("Lap {idx}: {lap} sec");
        }
    }

    fn finish(self){
        let total = self.laps.iter().sum::<i32>();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}

fn main(){
    struct_lesson_1();

    println!();

    struct_lesson_2();

    let mut race = Race::new("Monaco Grand Prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();

}