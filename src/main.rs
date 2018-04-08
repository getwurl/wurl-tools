#[macro_use]
extern crate clap;
extern crate regex;
extern crate scheduled_thread_pool;
#[macro_use]
extern crate lazy_static;

mod instruction;

use instruction::{Instruction, Command};
use clap::App;
use scheduled_thread_pool::ScheduledThreadPool;
use std::time::Duration;
use std::thread::{self, sleep, JoinHandle};

enum OpCode {
    Ping,
    Pong,
    Message,
    Close
}

fn main() {
    let yaml = load_yaml!("wurl-tools.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let pings = values_t!(matches, "ping", Instruction);
    //let pongs = values_t!(matches, "pong", Instruction);
    //let messages = values_t!(matches, "message", Instruction);
    //let closes = values_t!(matches, "close", Instruction);

    println!("pung: {:?}", pings);

    let pool = ScheduledThreadPool::with_name("wurl-tools {}", 1);

    if let Ok(instructions) = pings {
        println!("inst: {:?}", instructions);

        for instruction in instructions.iter() {
            println!("inst: {:?}", instruction);
            match instruction.command() {
                Command::DELAY => {
                    if let Some(message) = instruction.message() {
                        let cloned = message.clone();
                        pool.execute_after(*instruction.duration(), move || {
                            println!("/ping {}", cloned);
                        });
                    } else {
                        pool.execute_after(*instruction.duration(), || {
                            println!("/ping");
                        });
                    }
                }
                Command::INTERVAL => {
                    if let Some(message) = instruction.message() {
                        let cloned = message.clone();
                        pool.execute_at_fixed_rate(*instruction.duration(), *instruction.duration(), move || {
                            println!("/ping {}", cloned);
                        });
                    } else {
                        pool.execute_at_fixed_rate(*instruction.duration(), *instruction.duration(), || {
                            println!("/ping");
                        });
                    }
                }
            }
        }
    }

    loop {
        sleep(Duration::from_secs(1));
    }

    //for thread in threads {
    //    println!("Thread: {:?}", thread);
    //    thread.join().expect("Failed to join ping thread");
    //}
}
