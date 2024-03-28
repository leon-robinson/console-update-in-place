use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

fn main() {
    let mut stdout = stdout();

    for i in 0..=100 {
        print!("\rIn place string changing thing: {}%...", i);
        stdout.flush().unwrap();
        sleep(Duration::from_millis(10));
    }

    println!();

    let slots = 50;

    assert!(slots <= 100);

    for i in 0..slots {
        let mut s = String::from("[");

        for _ in 0..=i {
            s += "=";
        }

        for _ in i + 1..slots {
            s += ".";
        }

        s += "]";

        let progress_percent = if i == slots - 1 {
            100
        } else {
            (i + 1) * (100 / slots)
        };

        print!("\rTASK: {} {}%", s, progress_percent);
        stdout.flush().unwrap();
        sleep(Duration::from_millis(20));
    }

    println!();
}
