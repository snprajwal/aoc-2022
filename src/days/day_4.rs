pub fn solve(input: String) -> (String, String) {
    let v = input.lines().map(|x| {
        x.split(",")
            .flat_map(|y| y.split("-").map(|z| z.parse::<usize>().unwrap()))
            .collect::<Vec<usize>>()
    });

    (
        v.clone()
            .filter(|x| (x[0] <= x[2] && x[1] >= x[3]) || (x[0] >= x[2] && x[1] <= x[3]))
            .count()
            .to_string(),
        v.filter(|x| (x[0] <= x[2] && x[1] >= x[2]) || (x[2] <= x[0] && x[3] >= x[0]))
            .count()
            .to_string(),
    )
}
