use std::io;

#[derive(Debug)]
enum Result{
    Ok(i32),
    Err(String),
}

fn divide_in_two(num: i32) -> Result {
    if num % 2 == 0 {
        Result::Ok(num/2)
    }else{
        Result::Err(format!("cannot divide {num} into equal two parts"))
    }
}

fn main(){
    println!("Enter a number:\n");
    let mut num_string = String::new();
    let _ = io::stdin().read_line(&mut num_string).unwrap();
    let num = num_string.trim().parse::<i32>().unwrap();
    let output = divide_in_two(num);
    println!("Output = {:?}", output);
}