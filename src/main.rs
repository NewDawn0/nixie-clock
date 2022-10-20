// Setup
use colored::Colorize;
use chrono::Local;
use std::process;
use std::thread::sleep;
use simple_scheduler::{
    Duration, Time,
    Schedule, ScheduleAt, ScheduledTask, task
};
// Constants

// Main
fn main() {
    // Setup tasks
    let every_second = ScheduledTask::new(
        ScheduleAt::Interval(Duration::seconds(1)),
        task!(async {/*TODO ADD clock */})
    ).unwrap();
    let schedule = Schedule::builder()
        .tasks([
            every_second
        ])
        .wake_interval(Duration::seconds(1)).unwrap()
        .build();
    // Clear Terminal
    clear();
    // run
    schedule.run();
    loop {}
}
// fn clear
fn clear() {
    process::Command::new("clear").status().unwrap();
}
