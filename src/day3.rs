use std::u32;

#[aoc_generator(day3)]
pub fn gen(input: &str) -> Vec<u32> {
    input
        .trim()
        .lines()
        .map(|x| u32::from_str_radix(x, 2).unwrap())
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[u32]) -> u32 {
    let gamma = get_gamma(input);
    println!("true gamma: {:b}", gamma);
    let epsilon = !gamma & 0b111111111111;

    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn part2(input: &[u32]) -> u32 {
    let gamma = get_gamma(input);
    let epsilon = !gamma & 0b111111111111;

    let o2 = calc_ratings(gamma >> 11 != 0, input.to_vec());
    let co2 = calc_ratings(epsilon >> 11 != 0, input.to_vec());

    o2 * co2
}

fn get_gamma(input: &[u32]) -> u32 {
    let mut gamma = String::from("");

    for i in 0..12 {
        let mut sum0 = 0;
        let mut sum1 = 0;
        for line in input {
            let x = line.reverse_bits() >> 20;
            match x & (1 << i) != 0 {
                false => sum0 += 1,
                true => sum1 += 1,
            }
        }
        if sum0 > sum1 {
            gamma.push('0');
        } else {
            gamma.push('1');
        }
    }

    u32::from_str_radix(&gamma, 2).unwrap()
}

pub fn calc_ratings(msb: bool, input: Vec<u32>) -> u32 {
    let mut v = input.clone();

    v = v
        .into_iter()
        .filter(|x| ((x >> 11) != 0) == msb)
        .collect::<Vec<u32>>();

    let mut i = 10;
    while v.len() > 1 {
        let mut rate = get_gamma(&v);
        if !msb {
            rate = !rate & 0b111111111111;
        }
        println!("{:b}", rate);

        v = v
            .into_iter()
            .filter(|x| ((x >> i) & 1) == (rate >> i) & 1)
            .collect::<Vec<u32>>();
        i -= 1;
    }

    println!("{:b}, {}", v[0], v[0]);

    v[0]
}
