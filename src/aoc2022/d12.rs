use std::collections::BinaryHeap;
use std::io::BufRead;

#[derive(Clone, Debug, PartialEq, Eq, Ord)]
struct Path {
    steps: Vec<(usize, usize)>,
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(if self.steps.len() > other.steps.len() {
            std::cmp::Ordering::Less
        } else if self.steps.len() < other.steps.len() {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        })
    }
}

fn get_next_steps(pos: (usize, usize), map: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let h_cur = map[pos.1][pos.0];
    let mut res = Vec::new();
    for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let pn = (pos.0 as isize + d.0, pos.1 as isize + d.1);
        if pn.0 < 0 || pn.1 < 0 {
            continue;
        }
        let hn = map
            .get(pn.1 as usize)
            .and_then(|v| v.get(pn.0 as usize))
            .copied()
            .unwrap_or(0xff);
        if h_cur + 1 >= hn {
            res.push((pn.0 as usize, pn.1 as usize))
        }
    }
    res
}

pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let mut map = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let line = line.unwrap();
        let mut row = Vec::new();
        for (x, c) in line.trim().chars().enumerate() {
            let val = if c.is_lowercase() {
                c as u8
            } else if c == 'S' {
                start = (x, y);
                b'a'
            } else if c == 'E' {
                end = (x, y);
                b'z'
            } else {
                panic!("Invalid char '{}'", c)
            } - b'a';
            row.push(val);
        }
        map.push(row);
    }

    let mut shortest_paths = Vec::with_capacity(map.len());
    for l in map.iter() {
        let r = vec![usize::MAX; l.len()];
        shortest_paths.push(r);
    }

    let mut paths = BinaryHeap::new();
    let p0 = Path { steps: vec![start] };
    println!("{:?}", end);
    paths.push(p0);
    let res1 = 'outer: loop {
        let p = paths.pop().unwrap();
        let cpos = p.steps.last().unwrap().clone();
        let next = get_next_steps(cpos, &map);
        for n in next {
            if n == end {
                break 'outer p.steps.len();
            }
            let shortest = shortest_paths[n.1][n.0];
            if p.steps.len() + 1 < shortest {
                let mut np = p.clone();
                np.steps.push(n);
                shortest_paths[n.1][n.0] = np.steps.len();

                paths.push(np);
            }
        }
    };

    res1.into()
}
