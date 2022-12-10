use std::collections::VecDeque;

struct Bitplane {
    head: Pos,
    chain: Vec<Pos>,
    tail: Pos,
    plane: VecDeque<VecDeque<bool>>
}

#[derive(Clone, Copy)]
struct Pos {
    x: usize,
    y: usize
}

impl Bitplane {
    fn update_chain(&mut self) {
        if self.chain.len() > 0 {
            for idx in 0..self.chain.len() {
                if idx == 0 {
                    // initially grab from head
                    update_tail(self.head, &mut self.chain[0]);
                } else {
                    // then shuffle along the chain
                    update_tail(self.chain[idx-1], &mut self.chain[idx]);
                }
            }

            // finally update the real tail
            update_tail(self.chain[self.chain.len() - 1], &mut self.tail);
        } else {
            update_tail(self.head, &mut self.tail);
        }

        // set the plane at the tail position to true
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

            self.update_chain();
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

            self.update_chain();
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

                // update all chain links
                for link in self.chain.iter_mut() {
                    link.x += 1;
                }
            } else {
                self.head.x -= 1;
            }

            self.update_chain();
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

                // update all chain links
                for link in self.chain.iter_mut() {
                    link.y += 1;
                }
            } else {
                self.head.y -= 1;
            }

            self.update_chain();
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

    fn repr(&self) {
        for row in self.plane.iter() {
            row
                .iter()
                .map(|item| if *item {'#'} else {'.'})
                .for_each(|item| print!("{item}"));
            println!();
        }
    }
}

fn update_tail(head: Pos, tail: &mut Pos) {
    // calculate the difference between head and tail
    let x_diff: isize = head.x as isize - tail.x as isize;
    let y_diff: isize = head.y as isize - tail.y as isize;

    // with chains links can move in new ways, you have to check for both items being out of range
    if x_diff.abs() > 1 && y_diff.abs() > 1 {
        if x_diff > 0 { tail.x += 1 } else { tail.x -= 1 }
        if y_diff > 0 { tail.y += 1} else { tail.y -= 1 }
    } else {
        if x_diff.abs() > 1 {
            // left and right
            if tail.y != head.y {
                tail.y = head.y;
            }

            if x_diff > 0 { tail.x += 1 } else { tail.x -= 1 }
        } else if y_diff.abs() > 1 {
            // up and down
            if tail.x != head.x {
                tail.x = head.x;
            }

            if y_diff > 0 { tail.y += 1 } else { tail.y -= 1 }
        }
    }
}

fn positions_covered(chain: usize) -> usize {
    let data = include_str!("../data/9.input");

    // initial data structure involves position 0,0 and a true value at 0,0 on the bitplane
    let mut bitplane: Bitplane = Bitplane {
        head: Pos { x: 0, y: 0 },
        chain: vec![Pos { x: 0, y: 0}; chain],
        tail: Pos { x: 0, y: 0 },
        plane: VecDeque::from([VecDeque::from([true])])
    };

    for line in data.lines() {
        if let Some((direction, distance)) = line.split_once(' ') {
            bitplane.update(direction, distance)
        }
    }

    // visualise the bitplane
    if chain > 10 {
        bitplane.repr();
    }

    bitplane.count()
}


fn main() {
    println!("part one: {}", positions_covered(0));
    println!("part two: {}", positions_covered(8));
}
