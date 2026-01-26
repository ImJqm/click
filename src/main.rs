use chrono::{TimeDelta, prelude::*};
use std::{
    io::{self, Write},
    thread::{self, sleep},
    time::Duration,
};

fn main() {
    loop {
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().unwrap();

        let now_local: DateTime<Local> = Local::now();
        println!("The time is: {} ", now_local.format("%H:%M:%S"));

        let next_minute_start = now_local
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap()
            + chrono::Duration::seconds(1);

        thread::sleep(Duration::from_secs(1));
    }
}
