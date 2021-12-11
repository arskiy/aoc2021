use std::collections::HashSet;

#[aoc_generator(day8)]
pub fn gen(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input
        .lines()
        .map(|x| {
            let y: Vec<&str> = x.split(" | ").collect();
            let r1 = y[0].split_whitespace().map(|z| z.to_string()).collect();
            let r2 = y[1].split_whitespace().map(|z| z.to_string()).collect();
            (r1, r2)
        })
        .collect::<Vec<(Vec<String>, Vec<String>)>>()
}

#[aoc(day8, part1)]
pub fn part1(input: &Vec<(Vec<String>, Vec<String>)>) -> u32 {
    let mut sum = 0;
    for segments in input {
        for output in segments.1.clone() {
            let l = output.len();
            if l == 2 || l == 3 || l == 4 || l == 7 {
                sum += 1;
            }
        }
    }
    sum
}

#[aoc(day8, part2)]
pub fn part2(input: &Vec<(Vec<String>, Vec<String>)>) -> u32 {
    let mut sum = 0;
    for segments in input {
        let mut numbers: [HashSet<char>; 10] = Default::default();

        for seg in &segments.0 {
            let nchars = seg.len();
            let set: HashSet<char> = seg.chars().collect();
            match nchars {
                // cf
                2 => numbers[1] = set.clone(),
                // acf
                3 => numbers[7] = set.clone(),
                // bcdf
                4 => numbers[4] = set.clone(),
                // abcdefg
                7 => numbers[8] = set.clone(),
                _ => (),
            }

            // first match all singular numbers, then the shared ones
            if nchars == 5 {
                // 2 - acdeg
                // 3 - acdfg
                // 5 - abdfg

                if set.intersection(&numbers[1]).count() == 2 {
                    numbers[3] = set.clone();
                } else if numbers[4].intersection(&set).count() == 2 {
                    numbers[2] = set.clone();
                } else {
                    numbers[5] = set.clone();
                }

                if !numbers[2].is_empty() && !numbers[5].is_empty() {
                    numbers[3] = set.clone();
                } else if !numbers[3].is_empty() && !numbers[5].is_empty() {
                    numbers[2] = set.clone();
                } else if !numbers[3].is_empty() && !numbers[2].is_empty() {
                    numbers[5] = set.clone();
                }
            }

            if nchars == 6 {
                // 0 - abcefg
                // 6 - abdefg
                // 9 - abcdfg

                if set.intersection(&numbers[1]).count() == 1
                    || numbers[5].difference(&set).count() == 0
                {
                    numbers[6] = set.clone();
                } else if set.intersection(&numbers[4]).count() == 4
                    || numbers[4].difference(&set).count() == 0
                {
                    numbers[9] = set.clone();
                } else {
                    numbers[0] = set.clone();
                }

                /*
                if !numbers[9].is_empty() && !numbers[0].is_empty() {
                    numbers[6] = set.clone();
                } else if !numbers[6].is_empty() && !numbers[0].is_empty() {
                    numbers[9] = set.clone();
                } else if !numbers[9].is_empty() && !numbers[6].is_empty() {
                    numbers[0] = set.clone();
                }*/
            }

            /*
            // first match all singular numbers, then the shared ones
            if nchars == 5 {
                // 2 - acdeg
                // 3 - acdfg
                // 5 - abdfg

                if numbers[1].intersection(&set).count() == 2
                    || numbers[3].difference(&set).count() == 0
                {
                    numbers[3] = set.clone();
                } else if numbers[4].intersection(&set).count() == 2 {
                    numbers[2] = set.clone();
                } else {
                    numbers[5] = set.clone();
                }
            }

            if nchars == 6 {
                // 0 - abcefg
                // 6 - abdefg
                // 9 - abcdfg

                if numbers[1].intersection(&set).count() == 1 {
                    numbers[6] = set.clone();
                } else if numbers[4].intersection(&set).count() == 4
                    || numbers[4].difference(&set).count() == 0
                {
                    numbers[9] = set.clone();
                } else {
                    numbers[0] = set.clone();
                }
                /*
                if numbers[4].difference(&set).count() == 0 {
                    numbers[9] = set.clone();
                } else if numbers[5].difference(&set).count() == 0 {
                    numbers[6] = set.clone();
                } else {
                    numbers[0] = set.clone();
                }*/
            }
            */
        }

        println!("{:?}", numbers);

        let mut s = String::from("");
        for seg in &segments.1 {
            let set: HashSet<char> = seg.chars().collect();

            println!("{:?}", seg);

            s.push(
                std::char::from_digit((numbers.iter().position(|x| x == &set).unwrap()) as u32, 10)
                    .unwrap(),
            );
        }
        sum += s.parse::<u32>().unwrap();
    }
    sum
}
