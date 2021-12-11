#[aoc_generator(day11)]
pub fn gen(input: &str) -> Vec<u32> {
    let x = input.replace("\n", "");
    x.chars().map(|x| x.to_digit(10).unwrap()).collect()
}

#[aoc(day11, part1)]
pub fn part1(input: &[u32]) -> u32 {
    let mut octopi = input.clone().to_vec();
    let mut flashes = 0;

    for _ in 0..100 {
        let _ = octopi.iter_mut().map(|x| *x += 1).collect::<Vec<_>>();

        let mut reset = vec![];
        while let Some(x) = octopi.iter().position(|&x| x > 9) {
            flash(&mut octopi, (x % 10) as i32, (x / 10) as i32);
            flashes += 1;
            reset.push(x);
        }

        for i in reset {
            octopi[i] = 0;
        }
    }

    flashes
}

#[aoc(day11, part2)]
pub fn part2(input: &[u32]) -> u32 {
    let mut octopi = input.clone().to_vec();
    let mut step = 0;

    loop {
        if octopi.iter().all(|&x| x == 0) {
            return step;
        }

        let _ = octopi.iter_mut().map(|x| *x += 1).collect::<Vec<_>>();

        let mut reset = vec![];
        while let Some(x) = octopi.iter().position(|&x| x > 9) {
            flash(&mut octopi, (x % 10) as i32, (x / 10) as i32);
            reset.push(x);
        }

        for i in reset {
            octopi[i] = 0;
        }
        step += 1;
    }
}

pub fn flash(input: &mut Vec<u32>, x: i32, y: i32) {
    for (dx, dy) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    {
        if x + dx >= 0 && x + dx < 10 && y + dy >= 0 && y + dy < 10 {
            input[(dx + x + (dy + y) * 10) as usize] += 1;
        }
    }
    input[(x + y * 10) as usize] = 0;
}
