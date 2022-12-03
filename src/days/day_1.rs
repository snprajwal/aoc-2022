pub fn solve(input: String) -> (String, String) {
    let mut sums = input
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .flat_map(|v| v.parse::<usize>())
                .sum::<usize>()
        })
        .collect::<Vec<usize>>();
    sums.sort_by(|x, y| y.cmp(x));

    (
        sums[0].to_string(),
        sums.iter().take(3).sum::<usize>().to_string(),
    )
}
