#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<i64>> {
    input.lines()
        .map(|line| {
            line.split(" ")
                .map(|x| x.to_string().parse::<i64>().unwrap())
                .collect()
        })
    .collect()
}

#[inline]
fn extrapolate_for_p1(input: &[i64]) -> i64 {
    let mut seq = input.to_vec();
    let mut history = Vec::<i64>::new();
    while seq.iter().any(|x| *x != 0) {
        history.push(seq.last().unwrap().clone());
        seq = seq.windows(2)
            .map(|w| w[1] - w[0])
            .collect();
    }
    history.into_iter().sum()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[Vec<i64>]) -> i64 {
    input.iter()
        .map(|vec| extrapolate_for_p1(&vec))
        .sum()
}

fn extrapolate_for_p2(input: &[i64]) -> i64 {
    let mut seq = input.to_vec();
    let mut history = Vec::<i64>::new();
    while seq.iter().any(|x| *x != 0) {
        history.push(seq[0].clone());
        seq = seq.windows(2)
            .map(|w| w[1] - w[0])
            .collect();
    }
    history.into_iter().rev()
        .fold(0, |acc,el| {
            el - acc
        })
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[Vec<i64>]) -> i64 {
    input.iter()
        .map(|vec| extrapolate_for_p2(&vec))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn day9_parser() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(input.len(), 3);
        assert_eq!(input[0].len(), 6);
        assert_eq!(input[0][0], 0);
    }

    #[test]
    fn day9_extrapolate_for_p1() {
        let input1 = vec![0, 3, 6, 9, 12, 15];
        let ans1 = extrapolate_for_p1(&input1);
        assert_eq!(ans1, 18);
        let input2 = vec![1, 3, 6, 10, 15, 21];
        let ans2 = extrapolate_for_p1(&input2);
        assert_eq!(ans2, 28);
        let input3 = vec![10, 13, 16, 21, 30, 45];
        let ans3 = extrapolate_for_p1(&input3);
        assert_eq!(ans3, 68);
    }

    #[test]
    fn day9_solve_p1() {
        let input = input_generator(TEST_INPUT);
        let ans = solve_part1(&input);
        assert_eq!(ans, 114);
    }

    #[test]
    fn day9_extrapolate_for_p2() {
        let input1 = vec![0, 3, 6, 9, 12, 15];
        let ans1 = extrapolate_for_p2(&input1);
        assert_eq!(ans1, -3);
        let input2 = vec![1, 3, 6, 10, 15, 21];
        let ans2 = extrapolate_for_p2(&input2);
        assert_eq!(ans2, 0);
        let input3 = vec![10, 13, 16, 21, 30, 45];
        let ans3 = extrapolate_for_p2(&input3);
        assert_eq!(ans3, 5);
    }

    #[test]
    fn day9_solve_p2() {
        let input = input_generator(TEST_INPUT);
        let ans = solve_part2(&input);
        assert_eq!(ans, 2);
    }

}
