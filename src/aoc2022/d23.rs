use std::io::BufRead;

use crate::input;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Coord(i32, i32);
impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Elf {
    pos: Coord,
}

impl Elf {
    fn prepare_move(
        &self,
        round: usize,
        elves: &std::collections::HashMap<Coord, Elf>,
        new_elves: &mut std::collections::HashMap<Coord, Elf>,
    ) {
        let mut is_set = false;
        let neighbours = [
            Coord(-1, -1),
            Coord(0, -1),
            Coord(1, -1),
            Coord(1, 0),
            Coord(-1, 1),
            Coord(0, 1),
            Coord(1, 1),
            Coord(-1, 0),
        ];
        if neighbours
            .iter()
            .any(|x| elves.contains_key(&(*x + self.pos)))
        {
            for i in 0..4 {
                let dir_index = (round + i) % 4;
                let check_dir = match dir_index {
                    0 => {
                        // North
                        [Coord(-1, -1), Coord(0, -1), Coord(1, -1)]
                    }
                    1 => {
                        // South
                        [Coord(-1, 1), Coord(0, 1), Coord(1, 1)]
                    }
                    2 => {
                        // West
                        [Coord(-1, -1), Coord(-1, 0), Coord(-1, 1)]
                    }
                    3 => {
                        // East
                        [Coord(1, -1), Coord(1, 0), Coord(1, 1)]
                    }
                    _ => unreachable!(),
                };
                if !check_dir
                    .iter()
                    .map(|x| self.pos + *x)
                    .any(|x| elves.contains_key(&x))
                {
                    let new_pos = self.pos + check_dir[1];
                    if let Some(e) = new_elves.remove(&new_pos) {
                        new_elves.insert(e.pos, e);
                        new_elves.insert(self.pos, self.clone());
                    } else {
                        new_elves.insert(new_pos, self.clone());
                    }
                    is_set = true;
                    break;
                }
            }
        }
        if !is_set {
            new_elves.insert(self.pos, self.clone());
        }
    }
}

fn update_pos(elves: &mut std::collections::HashMap<Coord, Elf>) {
    for (k, e) in elves.iter_mut() {
        e.pos = *k;
    }
}

fn draw(elves: &std::collections::HashMap<Coord, Elf>) {
    let x_min = elves.iter().min_by_key(|x| x.0 .0).unwrap().0 .0;
    let x_max = elves.iter().max_by_key(|x| x.0 .0).unwrap().0 .0;
    let y_min = elves.iter().min_by_key(|x| x.0 .1).unwrap().0 .1;
    let y_max = elves.iter().max_by_key(|x| x.0 .1).unwrap().0 .1;
    let x_min = -3;
    let y_min = -2;
    let x_max = 10;
    let y_max = 9;
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let c = if elves.contains_key(&Coord(x, y)) {
                '#'
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!();
    }
}

pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let mut elves = std::collections::HashMap::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.unwrap().chars().enumerate() {
            if c == '#' {
                let pos = Coord(x as i32, y as i32);
                elves.insert(pos, Elf { pos });
            }
        }
    }
    let mut res1 = 0;
    let mut res2 = 0;
    for i in 0.. {
        let mut new_elves = std::collections::HashMap::new();
        for (_, e) in elves.iter() {
            e.prepare_move(i, &elves, &mut new_elves);
        }

        update_pos(&mut new_elves);
        if elves == new_elves {
            res2 = i + 1;
            break;
        }
        elves = new_elves;
        if i == 9 {
            let x_min = elves.iter().min_by_key(|x| x.0 .0).unwrap().0 .0;
            let x_max = elves.iter().max_by_key(|x| x.0 .0).unwrap().0 .0;
            let y_min = elves.iter().min_by_key(|x| x.0 .1).unwrap().0 .1;
            let y_max = elves.iter().max_by_key(|x| x.0 .1).unwrap().0 .1;

            let size = (x_max - x_min + 1) * (y_max - y_min + 1);
            res1 = size - elves.len() as i32;
        }
    }
    (res1, res2).into()
}
