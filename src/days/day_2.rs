use std::collections::HashMap;

pub fn solve(input: String) -> (String, String) {
    let scores_part_one: HashMap<&str, usize> = HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6),
    ]);

    let scores_part_two: HashMap<&str, usize> = HashMap::from([
        ("A X", 3),
        ("A Y", 4),
        ("A Z", 8),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 2),
        ("C Y", 6),
        ("C Z", 7),
    ]);

    (
        input
            .lines()
            .clone()
            .map(|x| scores_part_one.get(x).unwrap_or(&0))
            .sum::<usize>()
            .to_string(),
        input
            .lines()
            .map(|x| scores_part_two.get(x).unwrap_or(&0))
            .sum::<usize>()
            .to_string(),
    )
}
