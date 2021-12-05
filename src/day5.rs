#[derive(Debug)]
pub struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

#[aoc_generator(day5)]
pub fn gen(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|x| {
            let y = x.replace(" -> ", ",");
            let mut y = y.split(',').map(|y| y.parse::<i32>().unwrap());

            Line {
                x1: y.next().unwrap(),
                y1: y.next().unwrap(),
                x2: y.next().unwrap(),
                y2: y.next().unwrap(),
            }
        })
        .collect()
}

fn solve(input: &Vec<Line>, part1: bool) -> u32 {
    let mut ground = [[0; 1000]; 1000];

    for i in input {
        let x_sum = if i.x1 > i.x2 { -1 } else { 1 };
        let y_sum = if i.y1 > i.y2 { -1 } else { 1 };
        let mut x = i.x1.clone();
        let mut y = i.y1.clone();

        loop {
            ground[y as usize][x as usize] += 1;

            // vertical line
            if i.x1 == i.x2 {
                if y == i.y2 {
                    break;
                }

                y += y_sum;
            }
            // horizontal line
            else if i.y1 == i.y2 {
                if x == i.x2 {
                    break;
                }

                x += x_sum;
            }
            // diagonal
            else {
                if !part1 {
                    if x == i.x2 || y == i.y2 {
                        break;
                    }

                    x += x_sum;
                    y += y_sum;
                } else {
                    break;
                }
            }
        }
    }

    let mut sum = 0;

    for i in 0..ground.len() {
        for j in 0..ground[i].len() {
            if ground[i][j] >= 2 {
                sum += 1;
            }
        }
    }

    sum
}

#[aoc(day5, part1)]
pub fn part1(input: &Vec<Line>) -> u32 {
    solve(input, true)
}

#[aoc(day5, part2)]
pub fn part2(input: &Vec<Line>) -> u32 {
    solve(input, false)
}
