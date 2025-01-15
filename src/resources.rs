use bevy::prelude::*;

#[derive(Resource)]
pub struct WordTimer(pub Timer);

#[derive(Resource)]
pub struct Play(pub bool);

#[derive(Resource)]
pub struct Grid {
    matrix: Vec<Vec<Vec2>>,
}

impl Grid {
    pub fn new(x_w: f32, y_w: f32, jump: usize) -> Self {
        let x = (x_w / jump as f32) as usize;
        let y = (y_w / jump as f32) as usize;
        let grid = vec![vec![Vec2::new(0.0, 0.0); x]; y];
        Self {
            matrix: grid
                .iter()
                .enumerate()
                .map(|(i, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(j, _)| {
                            Vec2::new(
                                (-(x_w / 2.0) + (jump * i) as f32) as f32,
                                ((y_w / 2.0) - (jump * j) as f32) as f32,
                            )
                        })
                        .collect()
                })
                .collect(),
        }
    }
    pub fn get_indexs(&self, pos: Vec2) -> (usize, usize) {
        //si potrebbe lavorare di binary search...
        let x = self
            .matrix
            .iter()
            .enumerate()
            .find(|(_, row)| row[1].x == pos.x);
        let y = self.matrix[1]
            .iter()
            .enumerate()
            .find(|(_, p)| p.y == pos.y);
        (
            match x {
                Some((x, _)) => x,
                None => 0,
            },
            match y {
                Some((y, _)) => y,
                None => 0,
            },
        )
    }

    pub fn get(&self, x: usize, y: usize) -> Vec2 {
        *self
            .matrix
            .get(x)
            .unwrap_or(&self.matrix[0])
            .get(y)
            .unwrap_or(&self.matrix[0][0])
    }

    pub fn get_neiboor(&self, (x, y): (usize, usize), near: usize) -> Vec<Vec2> {
        let xn = [x + near, x, x.checked_sub(near).unwrap_or(x)];
        let yn = [y + near, y, y.checked_sub(near).unwrap_or(y)];
        let mut res = Vec::new();
        for i in xn {
            for j in yn {
                if (i, j) == (x, y) {
                    continue;
                }
                res.push(self.get(i, j));
            }
        }
        res
    }

    pub fn get_nearest(&self, x: f32, y: f32) -> Vec2 {
        //si potrebbe unire in un unico loop
        let mut low = 0;
        let mut high = self.matrix.len();
        while low < high {
            let n = ((high - low) / 2) + low;
            match self.get(n, 1).x > x {
                true => high = n,
                false => low = n,
            }
            if low + 1 == high {
                break;
            }
        }
        let delta_low = (self.get(low, 1).x - x).abs();
        let delta_high = (self.get(high, 1).x - x).abs();
        let right_x = match delta_low < delta_high {
            true => low,
            false => high,
        };
        let mut low = 0;
        let mut high = self.matrix[right_x].len();
        while low < high {
            let n = ((high - low) / 2) + low;
            match self.get(right_x, n).y > y {
                true => low = n,
                false => high = n,
            }
            if low + 1 == high {
                break;
            }
        }
        let delta_low = (self.get(low, 1).y - y).abs();
        let delta_high = (self.get(high, 1).y - y).abs();
        let right_y = match delta_low < delta_high {
            true => low,
            false => high,
        };
        self.get(right_x, right_y)
    }
}
