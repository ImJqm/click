use chrono::prelude::*;
use crossterm::{
    ExecutableCommand, execute,
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
    //enable_raw_mode()?;

    let mut stdout = io::stdout();

    stdout.execute(cursor::Hide)?;

    loop {
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().unwrap();

        let curr_window_size: WindowSize = window_size()?;

        let now_local: DateTime<Local> = Local::now();
        
        execute!(
            io::stdout(),
            MoveTo(curr_window_size.columns/2 -3, curr_window_size.rows/2 )
        );

        println!("{} ", now_local.format("%H:%M:%S"));


        //println!("{}", curr_window_size.rows);
        //println!("{}", curr_window_size.columns);

        let next_minute_start = now_local
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap()
            + chrono::Duration::seconds(1);

        thread::sleep(Duration::from_secs(1));
    }
}
