#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate rprompt;
extern crate scheduled_thread_pool;

mod instruction;

use std::process::exit;
use instruction::{Command, Instruction};
use clap::App;
use scheduled_thread_pool::ScheduledThreadPool;
use rprompt::read_reply;

enum OpCode {
    Ping,
    Pong,
    Message,
    Close,
}

fn main() {
    let yaml = load_yaml!("wurl-tools.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let pings = values_t!(matches, "ping", Instruction);
    let pongs = values_t!(matches, "pong", Instruction);
    let messages = values_t!(matches, "message", Instruction);
    let closes = values_t!(matches, "close", Instruction);

    let pool = ScheduledThreadPool::new(1);

    if let Ok(instructions) = pings {
        add_to_pool(instructions, OpCode::Ping, &pool);
    }

    if let Ok(instructions) = pongs {
        add_to_pool(instructions, OpCode::Pong, &pool);
    }

    if let Ok(instructions) = messages {
        add_to_pool(instructions, OpCode::Message, &pool);
    }

    if let Ok(instructions) = closes {
        add_to_pool(instructions, OpCode::Close, &pool);
    }

    // Pass stdin to stdout
    // This is to make pipelining possible
    loop {
        match read_reply() {
            Ok(input) => println!("{}", input),
            Err(error) => match error.kind() {
                std::io::ErrorKind::UnexpectedEof => {}
                _ => {
                    eprintln!("error: {}", error);
                    exit(1);
                }
            },
        }
    }
}

fn get_prefix(opcode: &OpCode) -> String {
    match *opcode {
        OpCode::Ping => String::from("/ping "),
        OpCode::Pong => String::from("/pong "),
        OpCode::Close => String::from("/close "),
        OpCode::Message => String::new(),
    }
}

fn add_to_pool(instructions: Vec<Instruction>, opcode: OpCode, pool: &ScheduledThreadPool) {
    for instruction in instructions.iter() {
        let prefix = get_prefix(&opcode);
        match *instruction.command() {
            Command::DELAY => {
                if let &Some(ref message) = instruction.message() {
                    let cloned = message.clone();
                    pool.execute_after(*instruction.duration(), move || {
                        println!("{}{}", prefix, cloned);
                    });
                } else {
                    pool.execute_after(*instruction.duration(), move || {
                        println!("{}", prefix);
                    });
                }
            }
            Command::INTERVAL => {
                if let &Some(ref message) = instruction.message() {
                    let cloned = message.clone();
                    pool.execute_at_fixed_rate(
                        *instruction.duration(),
                        *instruction.duration(),
                        move || {
                            println!("{}{}", prefix, cloned);
                        },
                    );
                } else {
                    pool.execute_at_fixed_rate(
                        *instruction.duration(),
                        *instruction.duration(),
                        move || {
                            println!("{}", prefix);
                        },
                    );
                }
            }
        }
    }
}
