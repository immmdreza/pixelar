use crate::prelude::Position;

pub struct GrowingSquare {
    n: isize,
    start_row: isize,
    start_column: isize,
    latest_square: Vec<(isize, isize)>,
}

impl GrowingSquare {
    pub fn new(row: isize, column: isize) -> Self {
        Self {
            n: 0,
            start_row: row,
            start_column: column,
            latest_square: vec![],
        }
    }
}

impl Iterator for GrowingSquare {
    type Item = Vec<Position>;

    fn next(&mut self) -> Option<Self::Item> {
        let new_square: Vec<(isize, isize)> = get_square(self.start_row, self.start_column, self.n);
        let result = new_square
            .iter()
            .filter(|x| !self.latest_square.contains(x))
            .filter_map(|x| Position::from_signed(x.0, x.1))
            .collect();

        self.n += 1;
        self.latest_square = new_square;
        Some(result)
    }
}

#[derive(Debug, Default)]
struct OddSeries {
    n: usize,
}

impl Iterator for OddSeries {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let res = Some((2 * self.n) + 1);
        self.n += 1;
        res
    }
}

fn get_square(x0: isize, y0: isize, offset: isize) -> Vec<(isize, isize)> {
    let mut res = vec![];
    for x in (x0 - offset)..=(x0 + offset) {
        for y in (y0 - offset)..=(y0 + offset) {
            res.push((x, y))
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd_series() {
        let os = OddSeries::default();
        assert_eq!(vec![1, 3, 5], os.take(3).collect::<Vec<usize>>())
    }

    #[test]
    fn test_square_grow_1() {
        for x in -1..=1 {
            for y in -1..=1 {
                println!("({}, {})", x, y)
            }
        }
    }

    #[test]
    fn test_square_grow_2() {
        let sg = GrowingSquare::new(0, 0);

        for t in sg.take(1) {
            println!("{:?}", t)
        }
    }
}
