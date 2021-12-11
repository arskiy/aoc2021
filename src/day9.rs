#[aoc_generator(day9)]
pub fn gen(input: &str) -> Vec<u32> {
    let x = input.replace("\n", "");
    x.chars().map(|x| x.to_digit(10).unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &[u32]) -> u32 {
    get_low_points(input)
        .iter()
        .fold(0, |acc, &x| acc + input[x as usize] + 1)
}

#[aoc(day9, part2)]
pub fn part2(input: &[u32]) -> u32 {
    let low = get_low_points(input);
    let mut basins = vec![];

    for l in low {
        basins.push(get_basin_size(input, l))
    }

    basins.sort();
    basins.iter().rev().take(3).product()
}

fn get_basin_size(input: &[u32], low: usize) -> u32 {
    let mut frontier = vec![low];
    let mut reached = vec![low];

    while !frontier.is_empty() {
        //println!("{:?}", frontier);
        let current = frontier.remove(0);
        let neighbors = get_neighbors((current % 100) as i32, (current / 100) as i32);
        //println!("neighbors: {:?}", neighbors);
        for neighbor in neighbors {
            if !reached.contains(&neighbor) && input[neighbor as usize] != 9 {
                frontier.insert(0, neighbor);
                reached.push(neighbor);
            }
        }
    }

    reached.len() as u32
}

fn get_low_points(input: &[u32]) -> Vec<usize> {
    let mut v = vec![];
    for x in 0..100 {
        for y in 0..100 {
            let v2 = get_neighbors(x, y);
            let mut i = v2.iter();
            if lower(
                input,
                input[(x + y * 100) as usize],
                i.next(),
                i.next(),
                i.next(),
                i.next(),
            ) {
                v.push((x + y * 100) as usize);
            }
        }
    }
    v
}

fn lower(
    input: &[u32],
    x: u32,
    y1: Option<&usize>,
    y2: Option<&usize>,
    y3: Option<&usize>,
    y4: Option<&usize>,
) -> bool {
    let y1 = input[*y1.unwrap()];
    let y2 = input[*y2.unwrap()];
    match (y3, y4) {
        (None, None) => y1 > x && y2 > x,
        (Some(&y3), None) => y1 > x && y2 > x && input[y3] > x,
        (Some(&y3), Some(&y4)) => y1 > x && y2 > x && input[y3] > x && input[y4] > x,
        _ => unimplemented!(),
    }
}

fn get_neighbors(x: i32, y: i32) -> Vec<usize> {
    let mut v = vec![];
    for (dx, dy) in [(-1, 0), (0, 1), (0, -1), (1, 0)].iter() {
        if x + dx >= 0 && x + dx < 100 && y + dy >= 0 && y + dy < 100 {
            v.push((x + dx + (dy + y) * 100) as usize);
        }
    }
    v
}
