use ndarray::prelude::*;
use petgraph::{prelude::UnGraphMap, algo::dijkstra};

#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord)]
pub enum GardenTile {
    Start,
    GardenPlot,
    Rock,
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Array2<GardenTile> {
    let parse_vec = input.lines()
        .map(|line| {
            line.bytes()
                .map(|ch| {
                    match ch {
                        b'.' => GardenTile::GardenPlot,
                        b'#' => GardenTile::Rock,
                        b'S' => GardenTile::Start,
                        _ => unreachable!("Encountered unexpected character.")
                    }
                })
            .collect::<Array1<GardenTile>>()
        })
    .collect::<Vec<Array1<GardenTile>>>();
    let shape = (parse_vec.len(), parse_vec[0].dim());
    let flat = parse_vec.iter().flatten().cloned().collect();
    let out = Array2::from_shape_vec(shape, flat).expect("Dimensions didn't line up");
    out
}

fn input_to_p1_graph(grid: &Array2<GardenTile>) -> UnGraphMap<(usize,usize), ()> {
    let mut out = UnGraphMap::<(usize,usize), ()>::with_capacity(grid.len(), 4*grid.len());
    let [outer_max, inner_max] = *grid.shape() else {unreachable!()};
    for i in 0..outer_max {
        for j in 0..inner_max {
            connect_edges(&mut out, grid, outer_max, inner_max, i, j);
        }
    }
    out
}

fn connect_edges(graph: &mut UnGraphMap<(usize,usize), ()>, grid: &Array2<GardenTile>,
    outer_max: usize, inner_max: usize, i: usize, j: usize) {
    if grid[(i,j)] == GardenTile::Rock {//Nothing connects to rocks
        return;
    }
    if i > 0 {
        if grid[(i-1,j)] != GardenTile::Rock {
            graph.add_edge((i,j), (i-1,j), ());
        }
    }
    if i < outer_max - 1 {
        if grid[(i+1,j)] != GardenTile::Rock {
            graph.add_edge((i,j), (i+1,j), ());
        }
    }
    if j > 0 {
        if grid[(i,j-1)] != GardenTile::Rock {
            graph.add_edge((i,j), (i,j-1), ());
        }
    }
    if j < inner_max - 1 {
        if grid[(i,j+1)] != GardenTile::Rock {
            graph.add_edge((i,j), (i,j+1), ());
        }
    }
}

pub fn p1_solver(input: &Array2<GardenTile>, goal_dist: Option<i64>) -> usize {
    let mut start = (0,0);
    let [outer_max, inner_max] = *input.shape() else {unreachable!()};
    for i in 0..outer_max {
        for j in 0..inner_max {
            if input[(i,j)] == GardenTile::Start {
                start = (i,j);
            }
        }
    }
    let graph = input_to_p1_graph(input);
    let ans = dijkstra(&graph, start, None, |_| 1);
    ans.iter()
        .filter(|(_,dist)| **dist <= goal_dist.unwrap_or(64))
        .filter(|(_,dist)| **dist % 2 == 0)
        .count()
}

#[aoc(day21, part1)]
pub fn solve_p1(input: &Array2<GardenTile>) -> usize {
    p1_solver(input, None)
}

//p2 solution : 617729401414635

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str =
"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    #[test]
    fn day21_p1() {
        let input = input_generator(TEST_INPUT);
        let ans = p1_solver(&input, Some(6));
        assert_eq!(ans, 16);
    }
}
