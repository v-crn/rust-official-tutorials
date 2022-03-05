fn main() {
    let mut i = 0;
    loop {
        println!("iteration: {}", i);
        i = i + 1;
        if i > 10 {
            println!("Stop!");
            break;
        }
    }
}
