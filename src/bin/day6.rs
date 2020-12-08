use std::collections::HashSet;

fn main() {
    let mut questions1: HashSet<char> = HashSet::new();
    let mut acc: HashSet<char> = HashSet::new();
    let mut count1 = 0;
    let mut count2 = 0;

    let mut first_person = true;
    include_str!("../../input/d6.txt").lines().for_each(|line| {
        if line == "" {
            count1 += questions1.len();
            count2 += acc.len();
            questions1 = HashSet::new();
            acc = HashSet::new();
            first_person = true;
            return;
        }

        let mut questions2: HashSet<char> = HashSet::new();
        for c in line.chars() {
            questions1.insert(c);
            questions2.insert(c);
        }

        if first_person {
            acc = questions2;
            first_person = false;
        } else {
            acc = acc.intersection(&questions2).cloned().collect();
        }
    });
    count1 += questions1.len();
    count2 += acc.len();

    println!("{}", count1);
    println!("{}", count2);
}
