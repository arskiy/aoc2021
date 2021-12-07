#[aoc_generator(day6)]
pub fn gen(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(fishes: &[u32]) -> usize {
    // Test input
    //let mut fishes = vec![3, 4, 3, 1, 2];
    let mut fishes = fishes.clone().to_vec();

    for _ in 0..80 {
        //println!("After {} days: {:?}", i, fishes);
        run_simulation(&mut fishes);
        //println!("After {} days: {}", i + 1, fishes.len());
    }

    fishes.len()
}

#[aoc(day6, part2)]
pub fn part2(fishes: &[u32]) -> usize {
    // Test input
    //let fishes = vec![3, 4, 3, 1, 2];

    let mut school = [0; 9];

    // init
    for fish in fishes {
        school[*fish as usize] += 1;
    }

    for _ in 0..256 {
        let temp = school[0];
        for j in 0..8 {
            school[j] = school[j + 1];
        }

        school[8] = temp;
        school[6] += temp;
    }

    school.iter().sum()
}

fn run_simulation(fishes: &mut Vec<u32>) {
    for i in 0..fishes.len() {
        if fishes[i] == 0 {
            fishes.push(8);
            fishes[i] = 7;
        }

        fishes[i] -= 1;
    }
}
