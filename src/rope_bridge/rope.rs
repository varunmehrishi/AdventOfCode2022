use std::cmp::Ordering;

use super::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rope<const N: usize> {
    pub knots: [(i32, i32); N],
}

impl<const N: usize> Rope<N> {
    pub fn new() -> Self {
        Self {
            knots: [Default::default(); N],
        }
    }

    pub fn tail(&self) -> (i32, i32) {
        self.knots[N - 1]
    }

    pub fn move_head(&mut self, d: Direction) {
        let (dx, dy) = Rope::<N>::get_differentials(d);

        self.knots[0].0 += dx;
        self.knots[0].1 += dy;

        self.update_tail(1);
    }

    fn get_differentials(d: Direction) -> (i32, i32) {
        match d {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn update_tail(&mut self, i: usize) {
        if i == N {
            return;
        }

        if self.is_tight(i) {
            self.tail_x_update_dir(i)
                .into_iter()
                .chain(self.tail_y_update_dir(i).into_iter())
                .map(Rope::<N>::get_differentials)
                .for_each(|(dx, dy)| {
                    self.knots[i].0 += dx;
                    self.knots[i].1 += dy;
                });
        }

        self.update_tail(i + 1)
    }

    fn is_tight(&self, i: usize) -> bool {
        let (hx, hy) = self.knots[i - 1];
        let (tx, ty) = self.knots[i];

        (hx.abs_diff(tx) > 1) || (hy.abs_diff(ty) > 1)
    }

    fn tail_x_update_dir(&self, i: usize) -> Option<Direction> {
        match self.knots[i - 1].0.cmp(&self.knots[i].0) {
            Ordering::Less => Some(Direction::Up),
            Ordering::Equal => None,
            Ordering::Greater => Some(Direction::Down),
        }
    }

    fn tail_y_update_dir(&self, i: usize) -> Option<Direction> {
        match self.knots[i - 1].1.cmp(&self.knots[i].1) {
            Ordering::Less => Some(Direction::Left),
            Ordering::Equal => None,
            Ordering::Greater => Some(Direction::Right),
        }
    }
}
