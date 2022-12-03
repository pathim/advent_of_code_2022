use std::io::BufRead;

fn get_prio(c: &char) -> u64 {
    *c as u64
        - if c.is_uppercase() {
            'A' as u64 - 27
        } else {
            'a' as u64 - 1
        }
}

fn str_to_u64(s: &str) -> u64 {
    s.chars()
        .map(|c| get_prio(&c))
        .map(|v| 1 << v)
        .fold(0, std::ops::BitOr::bitor)
}

pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let mut res1 = 0;
    let mut res2 = 0;
    let mut common_badge = u64::MAX;
    for (i, line) in input.lines().enumerate() {
        let line = line.unwrap();
        let line = line.trim();
        let l = line.len() / 2;
        let all = str_to_u64(line);
        common_badge &= all;
        if i % 3 == 2 {
            res2 += common_badge.trailing_zeros();
            common_badge = u64::MAX;
        }
        let s1 = str_to_u64(&line[..l]);
        let s2 = str_to_u64(&line[l..]);
        let common = s1 & s2;
        res1 += common.trailing_zeros();
    }
    (res1, res2).into()
}
