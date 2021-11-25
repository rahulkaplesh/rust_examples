#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Enter the outer loop!");
        'inner: loop {
            println!("Enter the inner loop!");
            break 'outer;
        }
        println!("Will never reach!");
    }
    println!("exited out of the loop");
}
