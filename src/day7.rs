#[aoc_generator(day7)]
pub fn gen(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

pub fn solve(crabs: &[u32], part1: bool) -> u32 {
    let mut smallest_sum = u32::MAX;

    for pos in 0..2000 {
        let mut sum = 0;
        for crab in crabs.iter() {
            let n = ((crab - pos) as i32).abs() as u32;
            if part1 {
                sum += n;
            } else {
                sum += (n * n + n) / 2;
            }
        }

        if smallest_sum > sum {
            smallest_sum = sum;
        }
    }

    smallest_sum
}

#[aoc(day7, part1)]
pub fn part1(crabs: &[u32]) -> u32 {
    solve(crabs, true)
}

#[aoc(day7, part2)]
pub fn part2(crabs: &[u32]) -> u32 {
    solve(crabs, false)
}
