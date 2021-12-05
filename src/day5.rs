#[derive(Debug)]
pub struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

#[aoc_generator(day5)]
pub fn gen(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|x| {
            let y = x.replace(" -> ", ",");
            let mut y = y.split(',').map(|y| y.parse::<u32>().unwrap());

            Line {
                x1: y.next().unwrap(),
                y1: y.next().unwrap(),
                x2: y.next().unwrap(),
                y2: y.next().unwrap(),
            }
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &Vec<Line>) -> u32 {
    let mut ground = [[0; 1000]; 1000];

    for i in input {
        if i.x1 == i.x2 {
            let r = if i.y1 > i.y2 {
                i.y2..=i.y1
            } else {
                i.y1..=i.y2
            };

            for j in r {
                ground[j as usize][i.x1 as usize] += 1;
            }
        } else if i.y1 == i.y2 {
            let r = if i.x1 > i.x2 {
                i.x2..=i.x1
            } else {
                i.x1..=i.x2
            };

            for j in r {
                ground[i.y1 as usize][j as usize] += 1;
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

#[aoc(day5, part2)]
pub fn part2(input: &Vec<Line>) -> u32 {
    let mut ground = [[0; 1000]; 1000];

    for i in input {
        if i.x1 == i.x2 {
            let r = if i.y1 > i.y2 {
                i.y2..=i.y1
            } else {
                i.y1..=i.y2
            };

            for j in r {
                ground[j as usize][i.x1 as usize] += 1;
            }
        } else if i.y1 == i.y2 {
            let r = if i.x1 > i.x2 {
                i.x2..=i.x1
            } else {
                i.x1..=i.x2
            };

            for j in r {
                ground[i.y1 as usize][j as usize] += 1;
            }
        } else {
            // handle all 4 cases of diagonals
            let x_sum = if i.x1 > i.x2 { -1 } else { 1 };
            let y_sum = if i.y1 > i.y2 { -1 } else { 1 };
            let mut x = i.x1 as i32;
            let mut y = i.y1 as i32;

            loop {
                ground[y as usize][x as usize] += 1;

                if x == i.x2 as i32 && y == i.y2 as i32 {
                    break;
                }

                x += x_sum;
                y += y_sum;
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
