use std::{io::BufRead, str::FromStr};

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
struct Cube(i32, i32, i32);
impl FromStr for Cube {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<i32> = s.split(',').filter_map(|x| x.parse().ok()).collect();
        if coords.len() != 3 {
            Err(format!("Not three coords: {}", s))
        } else {
            Ok(Self(coords[0], coords[1], coords[2]))
        }
    }
}
impl Cube {
    fn neighbours(&self) -> [Self; 6] {
        let mut res = [(); 6].map(|_| self.clone());
        res[0].0 += 1;
        res[1].0 -= 1;
        res[2].1 += 1;
        res[3].1 -= 1;
        res[4].2 += 1;
        res[5].2 -= 1;
        res
    }

    fn any_larger(&self, v: i32) -> bool {
        self.0 > v || self.1 > v || self.2 > v
    }
    fn any_smaller(&self, v: i32) -> bool {
        self.0 < v || self.1 < v || self.2 < v
    }
}

fn is_outside(cube: &Cube, outside: &std::collections::HashSet<Cube>, min: i32, max: i32) -> bool {
    outside.contains(cube) || cube.any_larger(max) || cube.any_smaller(min)
}

pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let cubes: std::collections::HashSet<Cube> =
        input.lines().map(|x| x.unwrap().parse().unwrap()).collect();
    let res1: usize = cubes
        .iter()
        .map(|c| c.neighbours().iter().filter(|n| !cubes.contains(n)).count())
        .sum();

    let min = cubes.iter().fold((i32::MAX, i32::MAX, i32::MAX), |r, c| {
        (r.0.min(c.0), r.1.min(c.1), r.2.min(c.2))
    });
    let max = cubes.iter().fold((i32::MIN, i32::MIN, i32::MIN), |r, c| {
        (r.0.max(c.0), r.1.max(c.1), r.2.max(c.2))
    });

    let min = min.0.min(min.1).min(min.2);
    let max = max.0.max(max.1).max(max.2);
    let mut outside = std::collections::HashSet::new();
    outside.insert(Cube(0, 0, 0));
    let mut filling = true;
    while filling {
        filling = false;
        let mut new_outside = std::collections::HashSet::new();
        for c in outside.iter() {
            for n in c.neighbours() {
                if !cubes.contains(&n) {
                    if !is_outside(&n, &outside, min, max) {
                        new_outside.insert(n);
                        filling = true;
                    }
                }
            }
        }
        outside.extend(new_outside.into_iter());
    }
    let res2: usize = cubes
        .iter()
        .map(|c| {
            c.neighbours()
                .iter()
                .filter(|n| is_outside(n, &outside, min, max))
                .count()
        })
        .sum();

    (res1, res2).into()
}
