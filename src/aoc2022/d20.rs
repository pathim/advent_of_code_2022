use std::io::BufRead;

#[derive(Clone, Debug)]
struct ListElem {
    value: i64,
    prev: usize,
    next: usize,
}

fn move_elem(list: &mut Vec<ListElem>, index: usize) {
    let len = list.len() as i64;
    let item = list[index].clone();
    if item.value == 0 {
        return;
    }
    list[item.prev].next = item.next;
    list[item.next].prev = item.prev;
    let mut target = index;
    let value = item.value.rem_euclid(len - 1);
    for _ in 0..value {
        target = list[target].next;
    }
    let idx_after = list[target].next;
    list[idx_after].prev = index;
    list[index].next = idx_after;
    list[index].prev = target;
    list[target].next = index;
}

fn get_result(list: &Vec<ListElem>) -> i64 {
    let zero_index = list.iter().position(|x| x.value == 0).unwrap();
    let mut cur = &list[zero_index];
    let mut res = 0;
    for _ in 0..3 {
        for _ in 0..1000 {
            cur = &list[cur.next];
        }
        res += cur.value;
    }
    res
}

fn create_list(numbers: &[i64], key: i64) -> Vec<ListElem> {
    let len = numbers.len();
    let mut list = Vec::with_capacity(len);
    for i in 0..len {
        let prev = if i == 0 { len - 1 } else { i - 1 };
        let mut next = i + 1;
        if next == len {
            next = 0;
        }
        let value = numbers[i] * key;
        list.push(ListElem { value, prev, next });
    }
    list
}

fn mix_list(list: &mut Vec<ListElem>) {
    for i in 0..list.len() {
        move_elem(list, i);
    }
}

pub fn f(file: std::fs::File) -> crate::AocResult {
    let input = std::io::BufReader::new(file);
    let numbers: Vec<i64> = input.lines().map(|l| l.unwrap().parse().unwrap()).collect();

    let mut list = create_list(&numbers, 1);
    mix_list(&mut list);
    let res1 = get_result(&list);

    let mut list2 = create_list(&numbers, 811589153);
    for _ in 0..10 {
        mix_list(&mut list2);
    }
    let res2 = get_result(&list2);

    (res1, res2).into()
}
