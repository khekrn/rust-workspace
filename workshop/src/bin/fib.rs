fn main() {
    let n: u64 = 622344;
    fib_iterative(n);
    //println!("fib({n}) = {}", fib(n));
}

fn fib_iterative(n: u64) {
    let (mut n1, mut n2): (u64, u64) = (0, 1);
    if n < 2 {
        println!("{:?}", n);
    }

    print!("{n1} {n2}");
    let mut n3: u64;
    let mut i: u64 = 2;
    while i < n {
        n3 = n1 + n2;
        print!(" {:?}", n3);
        n1 = n2;
        n2 = n3;
        i += 1;
    }
    println!()
}

#[allow(dead_code)]
fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}
