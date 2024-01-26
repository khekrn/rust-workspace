fn main(){
    println!("FizzBuzz program \n");
    for i in 1..30{
        fizzbuzz(i);
    }

    sample("Varun");
    sample("Varsha");
}

fn is_divisible(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    return lhs % rhs == 0;
}

fn fizzbuzz(n: u32){
    match (is_divisible(n, 3), is_divisible(n, 5)) {
       (true, true) => println!("fizzbuzz"),
       (true, false) => println!("fizz"),
       (false, true) => println!("buzz"),
       (false, false) => println!("{n}"),  
    }
}

fn sample(name: &str){
    match name {
        "Varun" => println!("Hello {name}"),
        "Kishore" => println!("Hello {name}"),
        _ => println!("Hello Rust !!")
    }
}