use std::error::Error;

///
/// Generates a `Vec` of `Vec`s containing `i32` values
/// denoting number sequence differences in the `list`
///
/// # Example output:
///
/// ```text
/// [
///     [2, 2, 2],
///     [0, 2, 4, 6],
///     [3, 3, 5, 9, 15],
///     [10, 13, 16, 21, 30, 45]
/// ]
/// ```
///
pub fn diffs(list: Vec<i32>) -> Vec<Vec<i32>> {
    let mut results: Vec<Vec<i32>> = vec![];

    let result: Vec<i32> = list
        .windows(2)
        .map(|slice| slice.last().unwrap() - slice.first().unwrap())
        .collect();

    if !result.iter().all(|&num| num == 0) {
        results.append(&mut diffs(result.clone()));
    }
    results.push(list);

    results
}

pub fn run(input: &str) -> Result<String, Box<dyn Error>> {
    let lines = input.lines();
    let num_lines: Vec<Vec<i32>> = lines
        .map(|line| line.split(' ').flat_map(str::parse::<i32>).collect())
        .collect::<Vec<_>>();

    let sum: i32 = num_lines
        .iter()
        .map(|line| {
            let diffs = diffs(line.to_vec());

            // start with zero since our 0 `Vec` is implicit
            let mut next_value: i32 = 0;

            // build first value
            diffs.iter().fold(0, |_, list| {
                next_value = list.first().unwrap() - next_value;
                next_value
            })
        })
        .sum();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn example_input() {
        let input = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

        let expected = "2".to_string();

        assert_eq!(expected, run(input).expect("should return expected value"));
    }
}
