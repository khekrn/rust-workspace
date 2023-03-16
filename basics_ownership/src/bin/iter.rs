struct Counter{
    count: u32,
}

impl Counter {
    fn new() -> Counter{
        Counter { count: 0 }
    }
}

impl Iterator for Counter{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        }else{
            None
        }
    }
}

fn main(){
    println!("Iterator Example !! \n");
    let c = Counter::new();
    for val in c{
        print!("{val}");
    }
    println!();
}

#[test]
fn iter_test(){
    let mut c = Counter::new();

    assert_eq!(c.next(), Some(1));
    assert_eq!(c.next(), Some(2));
    assert_eq!(c.next(), Some(3));
    assert_eq!(c.next(), Some(4));
    assert_eq!(c.next(), Some(5));
    assert_eq!(c.next(), None);
}