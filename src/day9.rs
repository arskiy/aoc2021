#[aoc_generator(day9)]
pub fn gen(input: &str) -> Vec<u32> {
    let x = input.replace("\n", "");
    x.chars().map(|x| x.to_digit(10).unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &Vec<u32>) -> u32 {
    let mut sum = 0;
    for i in 0..100 {
        for j in 0..100 {
            if i == 0 && j == 0 || i == 99 && j == 0 || i == 0 && j == 99 || i == 99 && j == 99 {
                continue;
            }

            let curr = input[i * 100 + j];

            if i == 0 {
                if lower(
                    curr,
                    input[(i + 1) * 100 + j],
                    input[i * 100 + (j - 1)],
                    input[i * 100 + (j + 1)],
                    None,
                ) {
                    sum += curr + 1;
                }
                continue;
            } else if i == 99 {
                if lower(
                    curr,
                    input[(i - 1) * 100 + j],
                    input[i * 100 + (j - 1)],
                    input[i * 100 + (j + 1)],
                    None,
                ) {
                    sum += curr + 1;
                }
                continue;
            } else if j == 0 {
                if lower(
                    curr,
                    input[(i + 1) * 100 + j],
                    input[(i - 1) * 100 + j],
                    input[i * 100 + (j + 1)],
                    None,
                ) {
                    sum += curr + 1;
                }
                continue;
            } else if j == 99 {
                if lower(
                    curr,
                    input[(i + 1) * 100 + j],
                    input[(i - 1) * 100 + j],
                    input[i * 100 + (j - 1)],
                    None,
                ) {
                    sum += curr + 1;
                }
                continue;
            }

            println!("i: {} j: {}", i, j);
            if lower(
                curr,
                input[(i + 1) * 100 + j],
                input[(i - 1) * 100 + j],
                input[i * 100 + (j - 1)],
                Some(input[i * 100 + (j + 1)]),
            ) {
                sum += curr + 1;
            }
        }
    }
    sum
}

pub fn lower(x: u32, y1: u32, y2: u32, y3: u32, y4: Option<u32>) -> bool {
    if let Some(y4) = y4 {
        y1 > x && y2 > x && y3 > x && y4 > x
    } else {
        y1 > x && y2 > x && y3 > x
    }
}
