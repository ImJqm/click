use chrono::prelude::*;
use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{self, MoveTo},
    style::{self, Print},
    terminal::{self, WindowSize, disable_raw_mode, enable_raw_mode, size, window_size},
};
use std::{
    io::{self, Write},
    thread::{self, sleep},
    time::Duration,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    stdout.execute(cursor::Hide)?;

    loop {
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().unwrap();

        let now_local: DateTime<Local> = Local::now();
        println!("{} ", now_local.format("%H:%M:%S"));

        let curr_window_size: WindowSize = window_size()?;

        println!("{}", curr_window_size.rows);

        let next_minute_start = now_local
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap()
            + chrono::Duration::seconds(1);

        thread::sleep(Duration::from_secs(1));
    }
}
