use nom::{
    IResult,
    Parser,
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, value},
    sequence::separated_pair,
    multi::{separated_list0,separated_list1},
    };

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct CubeColors {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeColors {
    fn set_color(&mut self, value: u32, color: Color) {
        *match color {
            Color::Red => &mut self.red,
            Color::Green => &mut self.green,
            Color::Blue => &mut self.blue,
        } = value;
    }
}

impl FromIterator<(u32,Color)> for CubeColors {
    fn from_iter<T: IntoIterator<Item = (u32, Color)>>(iter: T) -> Self {
        let mut cube_colors = Self::default();
        for (count, color) in iter {
            cube_colors.set_color(count, color);
        }
        cube_colors
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<CubeColors>> {
    match all_consuming(separated_list0(tag("\n"), parse_game))(input) {
        Ok((_,v)) => v,
        Err(e) => panic!("{}",e),
    }
}

fn parse_game(input: &str) -> IResult<&str, Vec<CubeColors>> {
    separated_pair(parse_game_id, tag(": "), parse_game_results)
        .map(|(_, v)| v)
        .parse(input)
}

fn parse_game_id(input: &str) -> IResult<&str, u32> {
    let (str, _) = tag("Game ")(input)?;
    let (out, num) = take_while1(char::is_numeric)(str)?;
    let num = match num.parse::<u32>() {
        Ok(val) => val,
        Err(e) => panic!("At input \"{}\" : {}", str, e),
    };
    Ok((out, num))
}

fn parse_game_results(input: &str) -> IResult<&str, Vec<CubeColors>> {
    separated_list0(tag("; "), parse_game_subset)(input)
}

fn parse_game_subset(input: &str) -> IResult<&str, CubeColors> {
    separated_list1(tag(", "), parse_color_val)
        .map(|el| el.into_iter().collect())
        .parse(input)
}
fn parse_color_val(input: &str) -> IResult<&str, (u32,Color)> {
    separated_pair(parse_val, tag(" "), parse_color)(input)
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    alt((
            value(Color::Red, tag("red")),
            value(Color::Green, tag("green")),
            value(Color::Blue, tag("blue")),
    ))(input)
}

fn parse_val(input: &str) -> IResult<&str, u32> {
    let (i, num) = take_while1(char::is_numeric)(input)?;
    let num = num.parse::<u32>().unwrap();
    Ok((i, num))
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
    use super::*;
    
    #[test]
    fn test_parse_color() {
        const INPUT: &str = "blue";
        let ans = parse_color(INPUT);
        assert_eq!(ans, Ok(("", Color::Blue)))
    }

    #[test]
    fn test_parse_val() {
        const INPUT: &str = "22 blue";
        let ans = parse_val(INPUT);
        assert_eq!(ans, Ok((" blue", 22)))
    }

    #[test]
    fn test_parse_color_val() {
        const INPUT1: &str = "3 blue";
        let ans1 = parse_color_val(INPUT1);
        assert_eq!(ans1, Ok(("", (3, Color::Blue))));
    }

    #[test]
    fn test_parse_game_subset() {
        const INPUT1: &str = "4 red";
        let ans = parse_game_subset(INPUT1);
        assert_eq!(ans, Ok(("", CubeColors{red: 4, green: 0, blue: 0})));
        const INPUT2: &str = "3 blue, 1 green, 4 red";
        let ans = parse_game_subset(INPUT2);
        assert_eq!(ans, Ok(("", CubeColors{red: 4, green: 1, blue: 3})))
    }

    #[test]
    fn test_parse_game() {
        const INPUT: &str = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let ans = parse_game(INPUT);
        assert_eq!(ans, Ok(("",
                    vec![
                    CubeColors {red: 0, green: 2, blue: 1},
                    CubeColors {red: 1, green: 3, blue: 4},
                    CubeColors {red: 0, green: 1, blue: 1},
                    ]
                    )))
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
        const EXAMPLE_CUBE_COLORS: CubeColors = CubeColors{red: 1, green: 1, blue: 1};
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

