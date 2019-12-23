use advent_of_code_2019::input::parse_list;
use advent_of_code_2019::intcode::{Machine, ProgramMachine, StepResult};
use std::collections::VecDeque;

fn main() {
    let program: Vec<i64> = parse_list(include_str!("input"), ',');
    println!("Answer to part 1: {}", part1(&program));
    println!("Answer to part 2: {}", part2(&program));
}

#[derive(Debug)]
struct Message {
    address: i64,
    x: i64,
    y: i64,
}

struct Computer {
    address: i64,
    machine: ProgramMachine,
    queue: VecDeque<Message>,
    waiting_for_input: bool,
}

impl Computer {
    fn new(address: i64, program: Vec<i64>) -> Computer {
        Computer {
            address,
            machine: ProgramMachine::new(program, vec![address]),
            queue: VecDeque::new(),
            waiting_for_input: false,
        }
    }

    fn step(&mut self) -> Option<Message> {
        match self.machine.step() {
            StepResult::Ok => None,
            StepResult::NeedInput => {
                if let Some(message) = self.queue.pop_front() {
                    self.waiting_for_input = false;
                    self.machine.add_input(message.x);
                    self.machine.add_input(message.y);
                } else {
                    self.waiting_for_input = true;
                    self.machine.add_input(-1);
                }
                None
            }
            StepResult::Output(address) => {
                self.waiting_for_input = false;
                let x = self.machine.run_to_output().unwrap();
                let y = self.machine.run_to_output().unwrap();
                Some(Message { address, x, y })
            }
            StepResult::Halt => panic!("computer {} halted unexpectedly", self.address),
        }
    }

    fn send(&mut self, message: Message) {
        self.queue.push_back(message);
    }

    fn is_idle(&self) -> bool {
        self.waiting_for_input && self.queue.is_empty()
    }
}

fn part1(program: &Vec<i64>) -> i64 {
    let mut computers = (0..50)
        .map(|address| Computer::new(address, program.clone()))
        .collect::<Vec<_>>();
    loop {
        for i in 0..computers.len() {
            if let Some(message) = computers[i].step() {
                if message.address == 255 {
                    return message.y;
                } else {
                    computers[message.address as usize].send(message);
                }
            }
        }
    }
}

fn part2(program: &Vec<i64>) -> i64 {
    let mut computers = (0..50)
        .map(|address| Computer::new(address, program.clone()))
        .collect::<Vec<_>>();
    let mut nat_message: Option<Message> = None;
    let mut last_nat_y: Option<i64> = None;
    loop {
        for i in 0..computers.len() {
            if let Some(message) = computers[i].step() {
                if message.address == 255 {
                    nat_message = Some(message);
                } else {
                    computers[message.address as usize].send(message);
                }
            }
        }
        let all_idle = computers.iter().all(|x| x.is_idle());
        if all_idle {
            if let Some(message) = nat_message.take() {
                if let Some(last_nat_y) = last_nat_y {
                    if last_nat_y == message.y {
                        return message.y;
                    }
                }
                last_nat_y = Some(message.y);
                computers[0].send(message);
            }
        }
    }
}
