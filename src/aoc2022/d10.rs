use std::{io::BufRead, str::FromStr};

enum Command {
    Noop,
    Addx(i64),
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Self::Noop);
        }
        let (_, v) = s.split_once(' ').ok_or("Not a valid command")?;
        let v = v.parse().map_err(|_| "Invalid number")?;
        Ok(Self::Addx(v))
    }
}

#[derive(Debug)]
enum State {
    Idle,
    Adding,
}

struct Cpu {
    state: State,
    x: i64,
    next_x: i64,
    cycle_count: usize,
    signal_strength_sum: i64,
    screen: [String; 6],
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            state: State::Idle,
            x: 1,
            next_x: 1,
            cycle_count: 0,
            signal_strength_sum: 0,
            screen: Default::default(),
        }
    }
    pub fn cycle(&mut self, cmd: &Option<Command>) -> i64 {
        let screen_pos = self.cycle_count % 40;
        let screen_line = self.cycle_count / 40;
        self.screen[screen_line] += if screen_pos.abs_diff(self.x as usize) <= 1 {
            "#"
        } else {
            " "
        };
        self.cycle_count += 1;
        let res = self.x * self.cycle_count as i64;
        match self.state {
            State::Idle => {
                if let Some(cmd) = cmd {
                    match *cmd {
                        Command::Noop => {}
                        Command::Addx(v) => {
                            self.next_x = self.x + v;
                            self.state = State::Adding;
                        }
                    }
                } else {
                    panic!("Not executing during idle");
                }
            }
            State::Adding => {
                if cmd.is_some() {
                    panic!("Executing command while in state {:?}", self.state);
                }
                self.x = self.next_x;
                self.state = State::Idle;
            }
        }
        if self.cycle_count % 40 == 20 {
            self.signal_strength_sum += res;
        }
        res
    }
    pub fn is_idle(&self) -> bool {
        matches!(self.state, State::Idle)
    }
    pub fn get_signal_strength_sum(&self) -> i64 {
        self.signal_strength_sum
    }
    pub fn get_screen(&self) -> String {
        self.screen.join("\n")
    }
}

pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let mut cpu = Cpu::new();
    for line in input.lines() {
        let line = line.unwrap();
        let cmd = Command::from_str(&line).unwrap();
        cpu.cycle(&Some(cmd));
        while !cpu.is_idle() {
            cpu.cycle(&None);
        }
    }

    (cpu.get_signal_strength_sum(), cpu.get_screen()).into()
}
