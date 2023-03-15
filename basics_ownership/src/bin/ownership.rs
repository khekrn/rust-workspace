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
    if x.len() > y.len() {
        return x;
    }else{
        return y;
    }
}

// fn lifetime_1<'a>(x: &'a str, y: &'a str) -> &'a str{
//     if x.len() > y.len() {
//         x
//     }else{
//         y
//     }
// }