use std::collections::VecDeque;

struct Bitplane {
    head: Pos,
    tail: Pos,
    plane: VecDeque<VecDeque<bool>>
}

struct Pos {
    x: usize,
    y: usize
}

impl Bitplane {
    fn update_tail(&mut self) {
        // TODO: be better

        if self.head.y > self.tail.y && self.head.y - self.tail.y > 1 {
            // down
            if self.tail.x != self.head.x {
                self.tail.x = self.head.x;
            }

            self.tail.y += 1;

        } else if self.tail.y > self.head.y && self.tail.y - self.head.y > 1 {
            // up
            if self.tail.x != self.head.x {
                self.tail.x = self.head.x;
            }

            self.tail.y -= 1;

        } else if self.head.x > self.tail.x && self.head.x - self.tail.x > 1 {
            // right
            if self.tail.y != self.head.y {
                self.tail.y = self.head.y;
            }

            self.tail.x += 1;

        } else if self.tail.x > self.head.x && self.tail.x - self.head.x > 1 {
            // left
            if self.tail.y != self.head.y {
                self.tail.y = self.head.y;
            }

            self.tail.x -= 1;
        }

        self.plane[self.tail.y][self.tail.x] = true;
    }

    fn move_r(&mut self, distance: usize) {
        for _ in 0..distance {
            if self.head.x == self.plane[self.head.y].len() - 1 {
                // append false to all lines
                for idx in 0..self.plane.len() {
                    self.plane[idx].push_back(false);
                }
            }

            self.head.x += 1;

            self.update_tail();
        }
    }

    fn move_d(&mut self, distance: usize) {
        for _ in 0..distance {
            if self.head.y == self.plane.len() - 1 {
                // create new row
                self.plane.push_back(VecDeque::new());

                // populate new row with as falses
                for _ in 0..self.plane[self.head.y].len() {
                    self.plane[self.head.y + 1].push_back(false);
                }
            }

            self.head.y += 1;

            self.update_tail();
        }
    }

    fn move_l(&mut self, distance: usize) {
        for _ in 0..distance {
            if self.head.x == 0 {
                // prepend false to all lines
                for idx in 0..self.plane.len() {
                    self.plane[idx].push_front(false);
                }

                self.tail.x += 1;
            } else {
                self.head.x -= 1;
            }

            self.update_tail();
        }
    }

    fn move_u(&mut self, distance: usize) {
        for _ in 0..distance {
            if self.head.y == 0 {
                self.plane.push_front(VecDeque::new());

                // populate new row with falses
                for _ in 0..self.plane[1].len() {
                    self.plane[self.head.y].push_back(false)
                }

                self.tail.y += 1;
            } else {
                self.head.y -= 1;
            }

            self.update_tail();
        }
    }

    fn update(&mut self, direction: &str, distance: &str) {
        if let Some(distance) = distance.parse::<usize>().ok() {
            match direction {
                "R" => self.move_r(distance),
                "D" => self.move_d(distance),
                "L" => self.move_l(distance),
                "U" => self.move_u(distance),
                _ => ()
            }
        }
    }

    fn count(self) -> usize {
        // counts true values in the bitplane
        self.plane.iter().map(|line| {
            line.iter().map(|val| if *val { 1 } else { 0 }).sum::<usize>()
        }).sum()
    }
}


fn positions_covered() -> usize {
    let data = include_str!("../data/9.input");

    // initial data structure involves position 0,0 and a true value at 0,0 on the bitplane
    let mut bitplane: Bitplane = Bitplane {
        head: Pos { x: 0, y: 0 },
        tail: Pos { x: 0, y: 0 },
        plane: VecDeque::from([VecDeque::from([true])])
    };

    for line in data.lines() {
        if let Some((direction, distance)) = line.split_once(' ') {
            bitplane.update(direction, distance)
        }
    }

    bitplane.count()
}


fn main() {
    println!("part one: {}", positions_covered());
}
