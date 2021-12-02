#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Forward,
}

#[aoc_generator(day2)]
pub fn gen(input: &str) -> Vec<(Direction, u32)> {
    let x = input
        .trim()
        .lines()
        .map(|l| l.split(' ').collect())
        .collect::<Vec<Vec<&str>>>();

    let mut v = vec![];
    for i in x {
        let dir = match i[0] {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            _ => unimplemented!(),
        };
        let n: u32 = i[1].parse().unwrap();
        v.push((dir, n));
    }

    v
}

#[aoc(day2, part1)]
pub fn part1(input: &[(Direction, u32)]) -> u32 {
    let mut depth = 0;
    let mut dist = 0;

    for i in input {
        match i.0 {
            Direction::Up => depth -= i.1,
            Direction::Down => depth += i.1,
            Direction::Forward => dist += i.1,
        }
    }

    depth * dist
}

#[aoc(day2, part2)]
pub fn part2(input: &[(Direction, u32)]) -> u32 {
    let mut angle = 0;
    let mut dist = 0;
    let mut depth = 0;

    for i in input {
        match i.0 {
            Direction::Up => angle -= i.1,
            Direction::Down => angle += i.1,
            Direction::Forward => {
                dist += i.1;
                depth += angle * i.1
            }
        }
    }
    println!("{} {}", dist, depth);
    depth * dist
}
