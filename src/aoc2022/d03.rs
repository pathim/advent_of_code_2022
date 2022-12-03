use std::collections::HashSet;
use std::io::BufRead;

fn get_prio(c: &char) -> u32 {
    *c as u32
        - if c.is_uppercase() {
            'A' as u32 - 27
        } else {
            'a' as u32 - 1
        }
}

pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let mut res1 = 0;
    let mut res2 = 0;
    let mut common_badge = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        let line = line.unwrap();
        let line = line.trim();
        let l = line.len() / 2;
        let all = line.chars().collect::<HashSet<_>>();
        common_badge = if common_badge.is_empty() {
            all
        } else {
            common_badge.intersection(&all).copied().collect()
        };
        if i % 3 == 2 {
            res2 += get_prio(common_badge.iter().next().unwrap());
            common_badge.clear();
        }
        let s1 = line[..l].chars().collect::<HashSet<_>>();
        let s2 = line[l..].chars().collect::<HashSet<_>>();
        let common = s1.intersection(&s2).next().unwrap();
        res1 += get_prio(common);
    }
    (res1, res2).into()
}
