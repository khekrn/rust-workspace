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
    return if random % 2 == 0 {
        CoinFlip::Head
    } else {
        CoinFlip::Tail
    }
}

fn main() {
    println!("You got: {:?}", flip_coin());
}
