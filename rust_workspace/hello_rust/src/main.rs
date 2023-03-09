use std::ops::Add;

fn main() {
    println!("Hello, Rust !!");
    let mut x_var = 6359;
    print!("{x_var}");
    while x_var != 1 {
        if x_var % 2 == 0 {
            x_var = x_var / 2;
        }else{
            x_var = 3 * x_var + 1;
        }
        print!(" -> {x_var}")
    }
    println!();

    println!("Array Sample");
    let mut arr:[i32; 10] = [0; 10];
    array_sample(&mut arr);

    dynamic_array(&mut [0; 5]);
    dynamic_array(&mut [0; 15]);

    tuple_sample(&("Kishore Karunakaran", 32));

    println!("Largest i32 = {:?}", generics_example(vec![5, 1, 4, 2, 10, 45, 11, 0].as_slice()));
    println!("Largest Alphabet = {:?}", generics_example(vec!['z', 'b', 'a', 'd', 'f'].as_slice()));

    string_fun();

    //struct fun
    let int_point = Point{x:10, y:20};
    let string_point = Point {x: String::from("Hello"), y: String::from("Rust")};

    println!("Int Struct = {:?}, sum = {:?}", int_point, int_point.sum());
    println!("String Struct = {:?}", string_point);


}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point<T>{
    x: T,
    y: T,
}

impl <T: Add<Output = T> + Copy> Point<T> {
    fn sum(&self) -> T{
        let result = self.x + self.y;
        return result; 
    }
}


fn array_sample(arr: &mut [i32; 10]){
    let mut index: usize = 0;
    for i in 0..6  {
        arr[index] = i*2;
        index +=1;
    }

    //For pretty print we can use #
    println!("Array values = {:#?}", arr)
}

fn dynamic_array<const SIZE: usize>(arr: &mut [u32; SIZE]){
    for i in 0..SIZE{
        arr[i] = rand::random::<u32>();
    }
    println!("Dynamic Array of size {SIZE} = {:?}", arr);
}

fn tuple_sample(result: &(&str, i32)){
    println!("Name = {:?} and Age = {:?}", result.0, result.1);
}

fn string_fun(){
    let a = "Hello";
    let b = " Rust !!";
    let c = String::from(a) + b;
    println!("Concat String = {:?}", c);
}

fn generics_example<T: std::cmp::PartialOrd>(list: &[T]) -> &T{
    let mut largest = &list[0];
    for item in list{
        if item > largest{
            largest = item;
        }
    }
    largest
}