use std::io::Read;

fn is_unique(v: &[u8]) -> bool {
    let mut r = 0;
    for c in v {
        let c = *c - b'a';
        let c = 1 << c;
        if r & c != 0 {
            return false;
        }
        r |= c;
    }
    true
}

fn find_res(input: &[u8], num: usize) -> usize {
    let mut seen = Vec::new();
    for (i, &v) in input.iter().enumerate() {
        seen.push(v);
        if seen.len() == num {
            if is_unique(&seen) {
                return i + 1;
            }
            seen.remove(0);
        }
    }
    unreachable!()
}

pub fn f(mut file: std::fs::File) -> crate::AocResult {
    let mut input = Vec::new();
    file.read_to_end(&mut input).ok();
    let res1 = find_res(&input, 4);
    let res2 = find_res(&input, 14);

    (res1, res2).into()
}
