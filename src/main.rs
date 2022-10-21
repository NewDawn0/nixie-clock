// Setupbul
use pad::{PadStr, Alignment};
use chrono::Local;
use colored::Colorize;
use std::process::{exit, Command};
use simple_scheduler::{Duration, Schedule, ScheduleAt, ScheduledTask, task};
// Constants
const DIGITS : [[&str; 11]; 8] = [
    ["","","","","","","","","","",""],
    ["┏━━━┓","  ╻  ","┏━━━┓","┏━━━┓","╻   ╻","┏━━━┓","┏━━━┓","┏━━━┓","┏━━━┓","┏━━━┓","     "],
    ["┃   ┃","  ┃  ","    ┃","    ┃","┃   ┃","┃    ","┃    ","    ┃","┃   ┃","┃   ┃","  ╻  "],
    ["┃   ┃","  ┃  ","    ┃","    ┃","┃   ┃","┃    ","┃    ","    ┃","┃   ┃","┃   ┃","     "],
    ["┃   ┃","  ┃  ","┏━━━┛","╺━━━┫","┗━━━┫","┗━━━┓","┣━━━┓","    ┃","┣━━━┫","┗━━━┫","     "],
    ["┃   ┃","  ┃  ","┃    ","    ┃","    ┃","    ┃","┃   ┃","    ┃","┃   ┃","    ┃","     "],
    ["┃   ┃","  ┃  ","┃    ","    ┃","    ┃","    ┃","┃   ┃","    ┃","┃   ┃","    ┃","  ╹  "],
    ["┗━━━┛","  ╹  ","┗━━━━","┗━━━┛","    ╹","┗━━━┛","┗━━━┛","    ╹","┗━━━┛","┗━━━┛","     "],
];
// Main
fn main() {
    // Setup
    let every_second = ScheduledTask::new(
        ScheduleAt::Interval(Duration::seconds(1)),
        task!(async { clock()})
    ).unwrap();
    let schedule = Schedule::builder()
        .tasks([
            every_second
        ])
        .wake_interval(Duration::seconds(1)).unwrap()
        .build();
    clear();
    // Run
    schedule.run();
    loop {}
}
// fn clear
fn clear() {
    Command::new("clear").status().unwrap();
}
// fn bulb
fn bulb (dig: &str, index: u8) -> String {
    let block = "|".to_string().truecolor(255,147,54);
    let sl = "\\".to_string().truecolor(255,147,54);
    let bsl = "/".to_string().truecolor(255,147,54);
    match index {
        0 => format!("{}", "  ---^---  ".to_string().truecolor(255,147,54)),
        1..=6 => format!(" {} {} {} ", block, dig.truecolor(249,212,102), block),
        7 => format!("  {}{}{}  ", sl, dig.truecolor(249,212,102), bsl),
        8 => format!("{}", "  |||||||  ".to_string().truecolor(140,130,140)),
        _ => {
            println!("Error aborting");
            exit(1);
        }
    }
}
// fn clock
fn clock () {
    clear();
    let now = Local::now();
    let time = now.format("%H:%M:%S");
    let mut index: u8 = 0;
    // print bulb
    for row in &DIGITS {
        for char in time.to_string().chars() {
            let column = match char {
                '0'..='9' => char as usize - '0' as usize,
                ':' => 10,
                _ => {
                    println!("Error aborting");
                    exit(1);
                }
            };
            print!("{} ", bulb(row[column], index));
        }
        index += 1;
        println!();
    }
    for _i in 0..8 {
        print!("{} ", bulb("", u8::from(8)));
    }
    println!();
    // Box
    let date = now.format("%A :: %d %b %Y");
    let _mid = format!("{}", date.to_string().pad_to_width_with_alignment(93, Alignment::Middle).truecolor(249,212,102));
    let block = format!("{}", "┃".to_string().truecolor(140, 110, 140));
    let mid = format!("{}{}{}", block, _mid, block);
    println!("{}", format!("{}", "┃▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔┃".to_string().truecolor(140,110,140)));
    println!("{}", mid);
    println!("{}", format!("{}", "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛".to_string().truecolor(140,110,140)));
}
