use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
struct Point{
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Self{
        Point { x, y }
    }
}

fn main(){
    mv_semantics();
    copy_clone();

    let x = String::from("Rust");
    let y = String::from("Go");
    let result = max(x.as_str(), y.as_str());
    println!("Result = {result:?}");

    life_time_struct();

    let res = generic_max_with_announcement(&x, &y, "Learning Lifetimes");
    println!("Result = {res:?}");
}

fn mv_semantics(){
    let s1 = String::from("Hello Rust !!");
    let s2 = s1;
    //Now S1 is no more valid
    println!("S2 = {s2}");
}

fn copy_clone(){
    let mut p1 = Point::new(10, 20);
    let p2 = p1;
    println!("X = {p1:?} and Y = {p2:?}");
    p1.x = 20;
    p1.y = 30;
    println!("X = {:?} and Y = {:?}", p1, p2);
}

fn max<'a>(x: &'a str, y: &'a str) -> &'a str {
    return if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn generic_max_with_announcement<'a, T>(x: &'a str, y: &'a str, announcement: T) -> &'a str 
where T: Display{
    println!("Attention Please : {}", announcement);
    if x.len() > y.len() {
        x
    }else{
        y
    }
}

struct ImportantExcerpt<'a>{
    part: &'a str,
}

impl <'a> ImportantExcerpt<'a>{
    fn return_part(&self, announcement: &str) -> &str{
        println!("Attention Please : {}", announcement);
        self.part
    }
}

fn life_time_struct(){
    let novel = String::from("Call me Kishore. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Cannot Find");
    let instance = ImportantExcerpt{
        part: first_sentence,
    };
    println!("Result = {:?}", instance.return_part("I have a static lifetime"));
}