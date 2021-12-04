#[aoc_generator(day1)]
pub fn gen(input: &str) -> Vec<u32> {
    input
        .trim()
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> u32 {
    let mut cnt = 0;
    let mut last = input[0];
    for n in input.iter() {
        if n > &last {
            cnt += 1;
        }
        last = *n;
    }
    cnt
}

#[aoc(day1, part2)]
pub fn part2(input: &[u32]) -> u32 {
    let mut cnt = 0;
    let mut last_window = 0;
    for window in input.windows(3) {
        let curr: u32 = window.iter().sum();
        if curr > last_window {
            cnt += 1;
        }
        last_window = curr;
    }
    cnt - 1
}
