#[aoc_generator(day7)]
pub fn gen(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: Vec<u32>) -> u32 {}
