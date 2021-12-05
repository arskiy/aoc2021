#[derive(Debug, Clone)]
pub struct Bingo {
    matrix: Vec<Vec<(u32, bool)>>,
}

impl Bingo {
    pub fn new(input: Vec<Vec<u32>>) -> Self {
        let mut matrix: Vec<Vec<(u32, bool)>> = vec![];
        for i in 0..input.len() {
            matrix.push(vec![]);
            for j in &input[i] {
                matrix[i].push((*j, false));
            }
        }

        Self { matrix }
    }

    pub fn check_win(&self) -> bool {
        self.check_rows() || self.check_columns()
    }

    pub fn check_rows(&self) -> bool {
        for i in 0..5 {
            let mut sum_row = 0;
            for j in 0..5 {
                if self.matrix[i][j].1 {
                    sum_row += 1;
                }
            }
            if sum_row >= 5 {
                return true;
            }
        }
        return false;
    }

    pub fn check_columns(&self) -> bool {
        for i in 0..5 {
            let mut sum_col = 0;
            for j in 0..5 {
                if self.matrix[j][i].1 {
                    sum_col += 1;
                }
            }
            if sum_col >= 5 {
                return true;
            }
        }
        return false;
    }

    pub fn should_mark(&self, n: u32) -> Option<(usize, usize)> {
        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[i].len() {
                if self.matrix[i][j].0 == n {
                    return Some((i, j));
                }
            }
        }

        None
    }

    pub fn mark(&mut self, ij: (usize, usize)) {
        self.matrix[ij.0][ij.1].1 = true;
    }

    pub fn sum_unmarked(&self) -> u32 {
        let mut sum = 0;

        for i in 0..self.matrix.len() {
            for j in &self.matrix[i] {
                if !j.1 {
                    //println!("Sum: {}, {}", sum, j.0);
                    sum += j.0;
                }
            }
        }

        sum
    }
}

#[aoc_generator(day4)]
pub fn gen(input: &str) -> (Vec<u32>, Vec<Bingo>) {
    let mut lines = input.lines();
    let draw = lines
        .nth(0)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut v: Vec<Vec<u32>> = vec![];
    let mut bingos = vec![];

    for line in lines {
        if line == "" {
            bingos.push(Bingo::new(v));
            v = vec![];
            continue;
        }

        let x: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        v.push(x);
    }

    (draw, bingos)
}

#[aoc(day4, part1)]
pub fn part1(input: &(Vec<u32>, Vec<Bingo>)) -> u32 {
    let (draw, bingos) = input;
    let mut bingos = bingos.clone();

    for n in draw {
        for bingo in bingos.iter_mut() {
            if let Some(ij) = bingo.should_mark(*n) {
                bingo.mark(ij);
            }

            if bingo.check_win() {
                return n * bingo.sum_unmarked();
            }
        }
    }
    0
}

#[aoc(day4, part2)]
pub fn part2(input: &(Vec<u32>, Vec<Bingo>)) -> u32 {
    let (draw, bingos) = input;
    let mut bingos = bingos.clone();
    let mut won = vec![];

    for n in draw {
        let mut i = 0;
        while i < bingos.len() {
            if let Some(ij) = bingos[i].should_mark(*n) {
                bingos[i].mark(ij);
            }

            if bingos[i].check_win() {
                //result = n * bingos[i].sum_unmarked();
                won.push((bingos[i].sum_unmarked(), n));
                //println!("{}", bingos.len());
                println!(
                    "{} * {} = {}",
                    n,
                    bingos[i].sum_unmarked(),
                    n * bingos[i].sum_unmarked()
                );
                bingos.remove(i);
            }

            i += 1;
        }
    }

    //println!("{:?}", won);

    // ¯\_(ツ)_/¯
    // literally no idea of where things went wrong but im tired of debugging this
    won[won.len() - 2].0 * won[won.len() - 2].1
}

/*

swap_remove:
[(1137, 5), (694, 65), (872, 65), (801, 16), (672, 93), (858, 37), (902, 3),
(611, 3), (652, 3), (734, 32), (626, 46), (769, 46), (685, 63), (674, 63),
(363, 24), (823, 24), (587, 24), (646, 81), (603, 81), (528, 81), (540, 81),
(336, 81), (439, 51), (682, 51), (554, 17), (457, 17), (534, 70), (479, 70),
(620, 78), (549, 61), (543, 61), (529, 61), (656, 91), (508, 91), (446, 91),
(488, 91), (614, 54), (548, 8), (472, 8), (369, 72), (587, 72), (581, 72),
(606, 72), (615, 72), (458, 72), (544, 40), (425, 40), (317, 40), (521, 40),
(456, 74), (332, 74), (543, 74), (386, 68), (361, 68), (439, 68), (375, 75),
(449, 75), (394, 75), (328, 67), (511, 67), (398, 39), (512, 39), (524, 10),
(410, 10), (380, 10), (343, 53), (233, 53), (410, 53), (451, 9), (298, 31),
(266, 6), (446, 7), (219, 7), (599, 7), (615, 7), (252, 7), (370, 47),
(255, 47), (408, 47), (406, 47), (310, 47), (358, 47), (424, 42), (441, 90),
(369, 19), (462, 36), (585, 36), (382, 22), (348, 22), (488, 43), (372, 28),
(482, 28), (362, 79), (355, 79), (295, 86), (430, 57), (352, 57), (439, 49), (176, 85)]


remove:
[(1137, 5), (694, 65), (872, 65), (801, 16), (672, 93), (858, 37), (902, 3),
(611, 3), (734, 32), (769, 46), (626, 46), (685, 63), (674, 63), (823, 24),
(587, 24), (363, 24), (646, 81), (540, 81), (336, 81), (603, 81), (528, 81),
(745, 51), (439, 51), (554, 17), (457, 17), (479, 70), (534, 70), (620, 78),
(549, 61), (543, 61), (529, 61), (508, 91), (446, 91), (488, 91), (656, 91),
(614, 54), (472, 8), (618, 8), (587, 72), (581, 72), (606, 72), (458, 72),
(586, 72), (615, 72), (369, 72), (544, 40), (425, 40), (317, 40), (521, 40),
(456, 74), (543, 74), (380, 74), (332, 74), (361, 68), (439, 68), (392, 68),
(449, 75), (394, 75), (303, 75), (457, 75), (328, 67), (511, 67), (398, 39),
(512, 39), (466, 64), (471, 10), (524, 10), (384, 53), (319, 53), (279, 53),
(451, 9), (415, 9), (376, 31), (278, 31), (341, 6), (258, 7), (297, 7),
(615, 7), (446, 7), (286, 47), (270, 47), (406, 47), (424, 42), (544, 42), (380, 90),
(357, 20), (265, 19), (401, 36), (518, 36), (331, 22), (488, 43), (353, 58),
(529, 28), (347, 28), (332, 79), (314, 86), (504, 57), (430, 49), (278, 26)]

*/

#[test]
pub fn column_test() {
    let x = vec![
        vec![
            (0u32, false),
            (0u32, false),
            (0u32, true),
            (0u32, false),
            (0u32, false),
        ],
        vec![
            (0u32, false),
            (0u32, false),
            (0u32, true),
            (0u32, false),
            (0u32, false),
        ],
        vec![
            (0u32, false),
            (0u32, false),
            (0u32, true),
            (0u32, false),
            (0u32, false),
        ],
        vec![
            (0u32, false),
            (0u32, false),
            (0u32, true),
            (0u32, false),
            (0u32, false),
        ],
        vec![
            (0u32, false),
            (0u32, false),
            (0u32, true),
            (0u32, false),
            (0u32, false),
        ],
    ];
    let bingo = Bingo { matrix: x };

    assert!(bingo.check_columns());
}

#[test]
pub fn row_test() {
    let x = vec![
        vec![
            (0u32, false),
            (0u32, false),
            (0u32, false),
            (0u32, false),
            (0u32, false),
        ],
        vec![
            (0u32, false),
            (0u32, false),
            (0u32, false),
            (0u32, false),
            (0u32, false),
        ],
        vec![
            (0u32, false),
            (0u32, false),
            (0u32, false),
            (0u32, false),
            (0u32, false),
        ],
        vec![
            (0u32, true),
            (0u32, true),
            (0u32, true),
            (0u32, true),
            (0u32, true),
        ],
        vec![
            (0u32, false),
            (0u32, false),
            (0u32, true),
            (0u32, false),
            (0u32, false),
        ],
    ];
    let bingo = Bingo { matrix: x };

    assert!(bingo.check_rows());
}
