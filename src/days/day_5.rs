use std::collections::VecDeque;

pub fn solve(input: String) -> (String, String) {
    // The original arrangement
    let mut crates: VecDeque<VecDeque<char>> = VecDeque::new();
    for _ in 0..9 {
        crates.push_back(VecDeque::new());
    }

    let (cargo, cmds) = input.split_once("\n\n").unwrap();

    // Parse the arrangement
    cargo
        // Replace the empty blocks with an int item
        // which will be filtered out later
        .replace("    ", "[0] ")
        // Properly pad the items between existing crates
        .replace("][0] ", "] [0]")
        .lines()
        .for_each(|x| {
            x.split_whitespace()
                .filter_map(|y| y.chars().nth(1))
                .enumerate()
                .for_each(|(i, y)| {
                    if !y.is_digit(10) {
                        crates[i].push_front(y);
                    }
                })
        });

    // Parse the move commands
    let moves = cmds.lines().map(|x| {
        x.split_whitespace()
            .filter_map(|y| y.parse::<usize>().ok())
            .map(|y| y - 1)
            .collect::<Vec<usize>>()
    });

    // Two copies of the original arrangement for each part
    let (mut crates1, mut crates2) = (crates.clone(), crates.clone());
    // Two solution strings
    let (mut s1, mut s2) = (String::default(), String::default());

    // Part 1
    moves.clone().for_each(|x| {
        for _ in 0..=x[0] {
            let item = crates1[x[1]].pop_back().unwrap();
            crates1[x[2]].push_back(item);
        }
    });

    for stack in crates1 {
        s1.push(*stack.back().unwrap());
    }

    // Part 2
    moves.for_each(|x| {
        let len = crates2[x[1]].len();
        let mut items = crates2[x[1]].split_off(len - x[0] - 1);
        crates2[x[2]].append(&mut items);
    });

    for stack in crates2 {
        s2.push(*stack.back().unwrap());
    }

    (s1, s2)
}
