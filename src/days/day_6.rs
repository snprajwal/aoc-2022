pub fn solve(input: String) -> (String, String) {
    let bytes = input.as_bytes();
    let mut present: [bool; 26] = [false; 26];

    // Part 1
    let s1 = (0..bytes.len() - 3)
        .find_map(|i| {
            for j in i..i + 4 {
                let k = (bytes[j] - 97) as usize;
                if present[k] {
                    present = [false; 26];
                    return None;
                }
                present[k] = true;
            }
            Some(i + 4)
        })
        .unwrap();

    // Part 2
    let s2 = (0..bytes.len() - 13)
        .find_map(|i| {
            for j in i..i + 14 {
                let k = (bytes[j] - 97) as usize;
                if present[k] {
                    present = [false; 26];
                    return None;
                }
                present[k] = true;
            }
            Some(i + 14)
        })
        .unwrap();

    (s1.to_string(), s2.to_string())
}
