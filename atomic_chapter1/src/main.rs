use std::{thread, ops::Range};

fn main() {
    thread_sample_with_vec();
    thread_sum_op(1..1000);
    thread_sample();
    thread_sum_op(1..10000043);
    scope_threads()
}

fn thread_sample(){
    let t1 = thread::spawn(process);
    let t2 = thread::spawn(process);

    println!("Hello from main thread = {:?}", thread::current().id());

    t1.join().unwrap();
    t2.join().unwrap();
}

fn thread_sum_op(range: Range<usize>){
    let numbers = Vec::from_iter(range);
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });
    let average = t.join().unwrap();
    println!("Average sum = {:?}", average);
}

fn thread_sample_with_vec(){
    let numbers = vec![1, 2, 3];
    thread::spawn(move || {
        for n in numbers{
            println!("{n:?}");
        }
    }).join().unwrap();
}

fn scope_threads(){
    let numbers = vec![1, 2, 3];
    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers{
                println!("{n}");
            }
        });
    }); 
}

fn process(){
    println!("Hello from another thread !!");
    let pid = thread::current().id();
    println!("This is my thread id = {pid:?}");
}
