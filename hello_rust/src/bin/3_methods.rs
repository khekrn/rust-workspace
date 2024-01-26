#[derive(Debug)]
struct Rectangle{
    height: u32,
    width: u32,
}

impl Rectangle {
    pub fn new(height: u32, width: u32) -> Self{
        return Rectangle{height, width};
    } 

    pub fn area(&self) -> u32 {
        return self.height * self.width;
    }

    pub fn incr_height(&mut self, delta: u32){
        self.height += delta;
    }

    pub fn incr_width(&mut self, delta: u32){
        self.width += delta;
    }
}

fn main(){
    let mut rec = Rectangle::new(12, 17);
    println!("Rectangle = {:?} and Area = {:?}", rec, rec.area());

    rec.incr_height(9);
    rec.incr_width(3);
    println!("Updated Rectangle = {:?} and Area = {:?}", rec, rec.area());
}