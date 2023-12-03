use nom::{bytes::complete::{take_while1, take_till}, IResult, multi::many0, error::Error};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CubeColors {
    red: u32,
    green: u32,
    blue: u32,
}

fn take_next_color_val<'a>(input: &'a str)  -> IResult<&'a str, (&'a str, u32)> {
    let (i,_) = take_till(char::is_alphanumeric)(input)?;
    let (i,num) = take_while1(char::is_numeric)(i)?;
    let (i,_) = take_while1(char::is_whitespace)(i)?;
    let (out,color) = take_while1(char::is_alphabetic)(i)?;
    let out = match take_till::<_,&str,Error<_>>(char::is_whitespace)(out) {
        Ok((val,_)) => val,
        Err(_) => "",
    };
    let num = match num.parse::<u32>() {
        Ok(val) => val,
        Err(_) => 0u32,
    };
    Ok((out,(color, num)))
}

fn find_color_val_pair(color_val_pair: &[(&str,u32)], color: &str) -> u32 {
    let ans = color_val_pair
        .iter()
        .filter(|(str,_)| *str ==  color)
        .next();
    match ans {
        Some((_,val)) => *val,
        None => 0,
    }
}

/* Expects a slice like " 3 blue, 4 red" */
fn block_to_cube_colors(input: &str) -> IResult<&str, CubeColors> {
    let (_,pairs) = many0(take_next_color_val)(input)?;
    let cubes = CubeColors {
        red: find_color_val_pair(&pairs, "red"),
        green: find_color_val_pair(&pairs, "green"),
        blue: find_color_val_pair(&pairs, "blue"),
    };
    Ok((input, cubes))
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<CubeColors>> {
    input
        .lines()
        .map(|line| {
            let game_results: String = line
                .chars()
                .skip(line.find(':').unwrap())
                .collect();
            game_results.split(';')
                .map(|block| {
                    match block_to_cube_colors(block) {
                        Ok((_,val)) => val,
                        Err(e) => panic!("{}", e),
                    }
                })
            .collect()
        })
    .collect()
}

fn does_game_pass_part1(game: &[CubeColors]) -> bool {
    !game.iter()
        .any(|el| {
            (el.red > 12)
                || (el.green > 13)
                || (el.blue > 14)
        })
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Vec<CubeColors>>) -> u32 {
    input.iter()
        .enumerate()
        .filter(|(_,x)| does_game_pass_part1(x))
        .map(|(n,_)| n as u32 + 1)
        .sum()
}

fn min_num_cubes_for_part2(game: &[CubeColors]) -> (u32,u32,u32) {
    game.iter()
        .fold((0,0,0), |(r,g,b), el| {
            let r_new = if el.red > r {el.red} else {r};
            let g_new = if el.green > g {el.green} else {g};
            let b_new = if el.blue > b {el.blue} else {b};
            (r_new, g_new, b_new)
        })
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Vec<CubeColors>>) -> u32 {
    input.iter()
        .map(|x| min_num_cubes_for_part2(x))
        .map(|(r,g,b)| r * g * b)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day2;

    use super::*;
    const TEST_COLOR_VAL: &str = " 3 blue, 4 red";

    #[test]
    fn test_take_next_color_val_1() {
        let ans1 = take_next_color_val(TEST_COLOR_VAL);
        assert_eq!(ans1, Ok((" 4 red", ("blue", 3))));
        let ans2 = take_next_color_val(ans1.unwrap().0);
        assert_eq!(ans2, Ok(("", ("red", 4))))
    }

    #[test]
    fn test_take_next_color_val_2() {
        const TEST_COLOR_VAL2: &str = ": 22 red, 5 green";
        let ans = take_next_color_val(TEST_COLOR_VAL2);
        assert_eq!(ans, Ok((" 5 green", ("red", 22))));
    }

    #[test]
    fn test_block_to_cube_colors() {
        let ans = block_to_cube_colors(TEST_COLOR_VAL);
        assert_eq!(ans, Ok((TEST_COLOR_VAL, CubeColors{red: 4, green: 0, blue: 3})))
    }

    const TEST_INPUT1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_input_generator() {
        let ans = input_generator(TEST_INPUT1);
        assert_eq!(ans.len(), 5);
        assert_eq!(ans[0].len(), 3);
        assert_eq!(ans[0][0], CubeColors{red: 4, green: 0, blue: 3});
        assert_eq!(ans[0][1], CubeColors{red: 1, green: 2, blue: 6});
        assert_eq!(ans[0][2], CubeColors{red: 0, green: 2, blue: 0});
        assert_eq!(ans[1].len(), 3);
        assert_eq!(ans[2].len(), 3);
        assert_eq!(ans[3].len(), 3);
        assert_eq!(ans[4].len(), 2);
    }

    #[test]
    fn test_does_game_pass_part1() {
        let test_game1: Vec<CubeColors> = vec![CubeColors{red: 1, green: 1, blue: 1}];
        assert_eq!(does_game_pass_part1(&test_game1), true);
        let test_game2: Vec<CubeColors> = vec![CubeColors{red: 13, green: 1, blue: 1}];
        assert_eq!(does_game_pass_part1(&test_game2), false);
        let test_game3: Vec<CubeColors> = vec![
            CubeColors{red: 20, green: 8, blue: 6},
            CubeColors{red: 4, green: 13, blue: 5},
            CubeColors{red: 1, green: 5, blue: 0},
        ];
        assert_eq!(does_game_pass_part1(&test_game3), false);
        let test_game4: Vec<CubeColors> = vec![CubeColors{red: 20, green: 20, blue: 20}];
        assert_eq!(does_game_pass_part1(&test_game4), false);
        let test_game5: Vec<CubeColors> = vec![CubeColors{red: 12, green: 13, blue: 14}];
        assert_eq!(does_game_pass_part1(&test_game5), true);
    }

    #[test]
    fn test_cube_colors_comparison() {
        const EXAMPLE_CUBE_COLORS: day2::CubeColors = CubeColors{red: 1, green: 1, blue: 1};
        /* Should be only greater than */
        assert_eq!(EXAMPLE_CUBE_COLORS < CubeColors{red: 2, green: 1, blue: 1}, true);
        assert_eq!(EXAMPLE_CUBE_COLORS > CubeColors{red: 2, green: 1, blue: 1}, false);
        /* Should be only less than */
        assert_eq!(EXAMPLE_CUBE_COLORS > CubeColors{red: 0,  green: 1, blue: 1}, true);
        assert_eq!(EXAMPLE_CUBE_COLORS < CubeColors{red: 0,  green: 1, blue: 1}, false);
        /* blah */
        assert_eq!(EXAMPLE_CUBE_COLORS < CubeColors{red: 0, green: 2, blue: 1}, false);
        assert_eq!(EXAMPLE_CUBE_COLORS < CubeColors{red: 2, green: 0, blue: 1}, true);
        assert_eq!(EXAMPLE_CUBE_COLORS > CubeColors{red: 0, green: 2, blue: 1}, true);
        assert_eq!(EXAMPLE_CUBE_COLORS > CubeColors{red: 2, green: 0, blue: 1}, false);
    }

    #[test]
    fn test_solve_day2p1() {
        let input = input_generator(TEST_INPUT1);
        let ans = solve_part1(&input);
        assert_eq!(ans, 8);
    }

    #[test]
    fn test_min_number_cubes_for_part2() {
        let input = input_generator(TEST_INPUT1);
        let ans = min_num_cubes_for_part2(&input[0]);
        assert_eq!(ans,(4,2,6));
    }

    #[test]
    fn test_solve_day2p2() {
        let input = input_generator(TEST_INPUT1);
        let ans = solve_part2(&input);
        assert_eq!(ans, 2286);
    }
}

