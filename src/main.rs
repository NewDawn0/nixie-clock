// Setup
use pad::{PadStr, Alignment};
use chrono::Local;
use colored::Colorize;
use std::{thread, process};
use simple_scheduler::{
    Duration, Time,
    Schedule, ScheduleAt, ScheduledTask, task
};
// Constants

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
    process::Command::new("clear").status().unwrap();
}
// fn clock
fn clock () {
    clear();
    let now = Local::now();
    let time = now.format("%H:%M:%S");
    // Box
    let date = now.format("%A :: %d %b %Y");
    let _mid = format!("{}", date.to_string().pad_to_width_with_alignment(68, Alignment::Middle).truecolor(249,212,102));
    let mid = format!("┃{}┃", _mid);
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("{}", mid);
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");
}
