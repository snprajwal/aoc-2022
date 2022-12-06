use std::collections::HashSet;

const LOWER: usize = b'a' as usize - 1;
const UPPER: usize = b'A' as usize - 1;

pub fn solve(input: String) -> (String, String) {
    (
        input
            .lines()
            .flat_map(|x| {
                let (a, b) = x.split_at(x.len() / 2);
                let h = a.chars().collect::<HashSet<char>>();
                b.chars().find(|y| h.contains(y)).map(|y| {
                    if y.is_ascii_lowercase() {
                        y as usize - LOWER
                    } else {
                        y as usize - UPPER + 26
                    }
                })
            })
            .sum::<usize>()
            .to_string(),
        input
            .lines()
            .collect::<Vec<&str>>()
            .chunks(3)
            .flat_map(|x| {
                let (a, b, c) = (x[0], x[1], x[2]);
                let h_a = a.chars().collect::<HashSet<char>>();
                let h_b = b
                    .chars()
                    .filter(|y| h_a.contains(y))
                    .collect::<HashSet<char>>();
                c.chars().find(|y| h_b.contains(y)).map(|y| {
                    if y.is_ascii_lowercase() {
                        y as usize - LOWER
                    } else {
                        y as usize - UPPER + 26
                    }
                })
            })
            .sum::<usize>()
            .to_string(),
    )
}
