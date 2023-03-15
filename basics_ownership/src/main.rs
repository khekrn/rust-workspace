fn main() {
    memory_layout();
}

fn memory_layout(){
    let mut s1 = String::from("Hello");
    s1.push(' ');
    s1.push_str("World");

    unsafe {
        let (cap, ptr, len): (usize, usize, usize) = std::mem::transmute(s1);
        println!("ptr = {ptr:#x}, len = {len}, capacity = {cap}");
    }
}
