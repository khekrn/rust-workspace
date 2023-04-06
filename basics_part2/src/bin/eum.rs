#[derive(Debug)]
enum CoinFlip {
    Head,
    Tail,
}

fn generate_random() -> i32 {
    4
}

fn flip_coin() -> CoinFlip {
    let random = generate_random();
    if random % 2 == 0 {
        return CoinFlip::Head;
    } else {
        return CoinFlip::Tail;
    }
}

fn main() {
    println!("You got: {:?}", flip_coin());
}
