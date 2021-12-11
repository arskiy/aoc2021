#[aoc_generator(day10)]
pub fn gen(input: &str) -> Vec<String> {
    input.lines().map(|x| x.trim().to_string()).collect()
}

#[aoc(day10, part1)]
pub fn part1(input: &[String]) -> u32 {
    get_corrupted(input).0
}

#[aoc(day10, part2)]
pub fn part2(input: &[String]) -> u32 {
    let indices = get_corrupted(input).1;
    let mut missing = vec![];

    for (i, line) in input.iter().enumerate() {
        if indices.contains(&i) {
            continue;
        }

        let mut stack = vec![];
        for c in line.chars() {
            match c {
                '(' => stack.push(c),
                '[' => stack.push(c),
                '{' => stack.push(c),
                '<' => stack.push(c),

                ')' | ']' | '}' | '>' => {
                    stack.pop();
                }

                '\n' => (),

                _ => unimplemented!(),
            }
        }

        let mut score = 0;

        for c in stack.iter().rev() {
            score *= 5;
            score += score_incomplete(opposite(*c));
        }

        missing.push(score);
    }

    missing.sort();
    println!("{:?}, {}, {}", missing, missing.len(), missing.len() / 2);
    missing[missing.len() / 2]
}

fn score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unimplemented!(),
    }
}

fn score_incomplete(c: char) -> u32 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unimplemented!(),
    }
}

fn opposite(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unimplemented!(),
    }
}

fn get_corrupted(input: &[String]) -> (u32, Vec<usize>) {
    let mut sum = 0;
    let mut indices = vec![];

    for (i, line) in input.iter().enumerate() {
        let mut stack = vec![];
        for c in line.chars() {
            match c {
                '(' => stack.push(c),
                '[' => stack.push(c),
                '{' => stack.push(c),
                '<' => stack.push(c),

                ')' | ']' | '}' | '>' => {
                    let x = stack.pop().unwrap();

                    if c != opposite(x) {
                        sum += score(c);
                        indices.push(i);
                        break;
                    }
                }

                '\n' => (),

                _ => unimplemented!(),
            }
        }
    }

    (sum, indices)
}
