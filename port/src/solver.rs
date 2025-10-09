use crate::math::lower_bound;
use crate::node::Node;
use crate::path::Path;
use crate::util::abs_diff;

#[derive(Clone)]
pub struct Solver {
    goal: Vec<u8>,
    tape: Vec<u8>,
    min_cost: i32,
    pointer: i32,
    max_pointer: i32,
    max_node_cost: i32,
    unique_cells: bool,
    zeros: Vec<i32>,
}

impl Solver {
    pub fn new(
        goal: &[u8],
        tape: Vec<u8>,
        pointer: i32,
        max_pointer: i32,
        max_node_cost: i32,
        unique_cells: bool,
    ) -> Self {
        let goal_vec = goal.to_vec();
        let mut min_cost = ((goal_vec.len() as i32).saturating_sub(1)) * 2;
        let repeats = goal_vec
            .windows(2)
            .filter(|pair| pair[0] == pair[1])
            .count() as i32;
        min_cost -= repeats;

        let zeros = (0..=max_pointer)
            .filter(|&i| {
                let idx = i as usize;
                idx < tape.len() && tape[idx] == 0
            })
            .collect();

        Self {
            goal: goal_vec,
            tape,
            min_cost,
            pointer,
            max_pointer,
            max_node_cost,
            unique_cells,
            zeros,
        }
    }

    pub fn solve(&mut self, max_cost: i32) -> Option<Path> {
        self.exhaustive(0, 0, self.pointer, max_cost - self.min_cost)
    }

    fn exhaustive(&mut self, cost: i32, start: usize, pointer: i32, max_cost: i32) -> Option<Path> {
        // lots of code repetition for the sake of speed :/
        // let zeros = (0..=self.max_pointer).filter(|&x| self.tape[x as usize] == 0).collect::<Vec<_>>();
        if start == self.goal.len() {
            return Some(Path::new());
        }

        let next_cost = if start + 1 == self.goal.len() {
            0
        } else if self.goal[start] == self.goal[start + 1] {
            1
        } else {
            2
        };

        let mut min_path: Option<Path> = None;
        let i_max = (max_cost - cost).min(self.max_node_cost);
        if i_max <= 0 {
            return None;
        }

        // stay on same pointer
        if pointer >= 0 {
            let idx = pointer as usize;
            if idx < self.tape.len() {
                let ncost = abs_diff(self.goal[start], self.tape[idx]) + 1;
                if ncost <= i_max {
                    let node = Node::new(pointer, ncost);
                    let tval = self.tape[idx];
                    let old_zeros = self.zeros.clone();
                    if tval == 0 {
                        self.recompute_zeros();
                    }
                    self.tape[idx] = self.goal[start];
                    let subpath = self.exhaustive(
                        cost + ncost,
                        start + 1,
                        node.pointer,
                        max_cost + next_cost,
                    );
                    self.tape[idx] = tval;
                    self.zeros = old_zeros;

                    if let Some(mut subpath) = subpath {
                        let better = min_path
                            .as_ref()
                            .map(|p| subpath.cost() + ncost < p.cost())
                            .unwrap_or(true);
                        let unique_ok = !self.unique_cells || !subpath.contains(&node);
                        if better && unique_ok {
                            subpath.add_first(node);
                            min_path = Some(subpath);
                        }
                    }
                }
            }
        }

        // move left
        if pointer > 0 {
            for i in 1..i_max {
                if i > pointer {
                    break;
                }
                let target = pointer - i;
                let idx = target as usize;
                if idx >= self.tape.len() {
                    break;
                }
                let ncost = abs_diff(self.goal[start], self.tape[idx]) + i + 1;
                if ncost <= i_max {
                    let node = Node::new(target, ncost);
                    let tval = self.tape[idx];
                    let old_zeros = self.zeros.clone();
                    if tval == 0 {
                        self.recompute_zeros();
                    }
                    self.tape[idx] = self.goal[start];
                    let subpath = self.exhaustive(
                        cost + ncost,
                        start + 1,
                        node.pointer,
                        max_cost + next_cost,
                    );
                    self.tape[idx] = tval;
                    self.zeros = old_zeros;

                    if let Some(mut subpath) = subpath {
                        let better = min_path
                            .as_ref()
                            .map(|p| subpath.cost() + ncost < p.cost())
                            .unwrap_or(true);
                        let unique_ok = !self.unique_cells || !subpath.contains(&node);
                        if better && unique_ok {
                            subpath.add_first(node);
                            min_path = Some(subpath);
                        }
                    }
                }
            }
        }

        // move right
        if pointer < self.max_pointer {
            for i in 1..i_max {
                if pointer + i > self.max_pointer {
                    break;
                }
                let target = pointer + i;
                let idx = target as usize;
                if idx >= self.tape.len() {
                    break;
                }
                let ncost = abs_diff(self.goal[start], self.tape[idx]) + i + 1;
                if ncost <= i_max {
                    let node = Node::new(target, ncost);
                    let tval = self.tape[idx];
                    let old_zeros = self.zeros.clone();
                    if tval == 0 {
                        self.recompute_zeros();
                    }
                    self.tape[idx] = self.goal[start];
                    let subpath = self.exhaustive(
                        cost + ncost,
                        start + 1,
                        node.pointer,
                        max_cost + next_cost,
                    );
                    self.tape[idx] = tval;
                    self.zeros = old_zeros;

                    if let Some(mut subpath) = subpath {
                        let better = min_path
                            .as_ref()
                            .map(|p| subpath.cost() + ncost < p.cost())
                            .unwrap_or(true);
                        let unique_ok = !self.unique_cells || !subpath.contains(&node);
                        if better && unique_ok {
                            subpath.add_first(node);
                            min_path = Some(subpath);
                        }
                    }
                }
            }
        }

        // zip to previous zero [<]
        if pointer > 0 {
            let mut pj = 0;
            while pj <= pointer {
                let idx = (pointer - pj) as usize;
                if idx >= self.tape.len() || self.tape[idx] != 0 {
                    break;
                }
                pj += 1;
            }
            if pj <= pointer {
                let zcost = 3 + pj;
                let search = pointer - pj;
                let pz_idx = lower_bound(&self.zeros, search) as isize - 1;
                if pz_idx >= 0 {
                    let prev_zero = self.zeros[pz_idx as usize];

                    // from prev_zero going left
                    for i in 1..i_max {
                        if i >= i_max - 3 {
                            break;
                        }
                        if i > prev_zero {
                            break;
                        }
                        let target = prev_zero - i;
                        if target < 0 {
                            break;
                        }
                        let idx = target as usize;
                        if idx >= self.tape.len() {
                            break;
                        }
                        let ncost = abs_diff(self.goal[start], self.tape[idx]) + i + 1 + zcost;
                        if ncost <= i_max {
                            let node = Node::new(target, ncost);
                            let tval = self.tape[idx];
                            let old_zeros = self.zeros.clone();
                            if tval == 0 {
                                self.recompute_zeros();
                            }
                            self.tape[idx] = self.goal[start];
                            let subpath = self.exhaustive(
                                cost + ncost,
                                start + 1,
                                node.pointer,
                                max_cost + next_cost,
                            );
                            self.tape[idx] = tval;
                            self.zeros = old_zeros;

                            if let Some(mut subpath) = subpath {
                                let better = min_path
                                    .as_ref()
                                    .map(|p| subpath.cost() + ncost < p.cost())
                                    .unwrap_or(true);
                                let unique_ok = !self.unique_cells || !subpath.contains(&node);
                                if better && unique_ok {
                                    subpath.add_first(node);
                                    min_path = Some(subpath);
                                }
                            }
                        }
                    }

                    // from prev_zero going right towards pointer
                    for i in 1..i_max {
                        if i >= i_max - 3 {
                            break;
                        }
                        let limit = (pointer - prev_zero - 3) >> 1;
                        if i > limit {
                            break;
                        }
                        let target = prev_zero + i;
                        if target > self.max_pointer {
                            break;
                        }
                        let idx = target as usize;
                        if idx >= self.tape.len() {
                            break;
                        }
                        let ncost = abs_diff(self.goal[start], self.tape[idx]) + i + 1 + zcost;
                        if ncost <= i_max {
                            let node = Node::new(target, ncost);
                            let tval = self.tape[idx];
                            let old_zeros = self.zeros.clone();
                            if tval == 0 {
                                self.recompute_zeros();
                            }
                            self.tape[idx] = self.goal[start];
                            let subpath = self.exhaustive(
                                cost + ncost,
                                start + 1,
                                node.pointer,
                                max_cost + next_cost,
                            );
                            self.tape[idx] = tval;
                            self.zeros = old_zeros;

                            if let Some(mut subpath) = subpath {
                                let better = min_path
                                    .as_ref()
                                    .map(|p| subpath.cost() + ncost < p.cost())
                                    .unwrap_or(true);
                                let unique_ok = !self.unique_cells || !subpath.contains(&node);
                                if better && unique_ok {
                                    subpath.add_first(node);
                                    min_path = Some(subpath);
                                }
                            }
                        }
                    }

                    // roll to previous zero [.<]
                    let mut run_len = pointer - pj - prev_zero;
                    while pj < 1
                        && pointer - pj < self.max_pointer
                        && self.tape[(pointer - pj + 1) as usize] != 0
                        && run_len < (self.goal.len() - start) as i32
                    {
                        pj -= 1;
                        run_len += 1;
                    }
                    while run_len >= 4 && run_len as usize <= self.goal.len() - start {
                        let target = pointer - pj;
                        let idx = target as usize;
                        if idx >= self.tape.len() {
                            break;
                        }
                        let ncost = abs_diff(self.goal[start], self.tape[idx]) + pj.abs() + 1;
                        if ncost <= i_max {
                            let mut subpath = Path::new();
                            subpath.add_last(Node::new_with_rolling(target, ncost + 3));
                            let mut dcost = next_cost;
                            let mut valid = true;
                            for i in 1..run_len {
                                let cell = pointer - pj - i;
                                if cell < 0 {
                                    valid = false;
                                    break;
                                }
                                let cidx = cell as usize;
                                if cidx >= self.tape.len() {
                                    valid = false;
                                    break;
                                }
                                let delta =
                                    abs_diff(self.goal[start + i as usize], self.tape[cidx]);
                                if delta > i_max {
                                    valid = false;
                                    break;
                                }
                                subpath.add_last(Node::new_with_rolling(cell, delta));
                                dcost += if start + i as usize + 1 == self.goal.len() {
                                    0
                                } else if self.goal[start + i as usize]
                                    == self.goal[start + i as usize + 1]
                                {
                                    1
                                } else {
                                    2
                                };
                            }
                            if valid && subpath.len() as i32 == run_len {
                                let mut saved = Vec::with_capacity(run_len as usize);
                                for node in subpath.iter() {
                                    saved.push(self.tape[node.pointer as usize]);
                                }
                                for (offset, node) in subpath.iter().enumerate() {
                                    self.tape[node.pointer as usize] = self.goal[start + offset];
                                }
                                if let Some(subpath2) = self.exhaustive(
                                    cost + subpath.cost(),
                                    start + run_len as usize,
                                    prev_zero,
                                    max_cost + dcost,
                                ) {
                                    let better = min_path
                                        .as_ref()
                                        .map(|p| subpath.cost() + subpath2.cost() < p.cost())
                                        .unwrap_or(true);
                                    if better {
                                        let mut combined = subpath.clone();
                                        combined.extend_back(&subpath2);
                                        min_path = Some(combined);
                                    }
                                }
                                for (node, value) in subpath.iter().zip(saved.iter()) {
                                    self.tape[node.pointer as usize] = *value;
                                }
                            }
                        }
                        pj += 1;
                        run_len -= 1;
                    }
                }
            }
        }

        // zip to next zero [>]
        if pointer <= self.max_pointer {
            let mut nj = 0;
            while pointer + nj <= self.max_pointer {
                let idx = (pointer + nj) as usize;
                if idx >= self.tape.len() || self.tape[idx] != 0 {
                    break;
                }
                nj += 1;
            }
            if pointer + nj <= self.max_pointer {
                let zcost = 3 + nj;
                let nz_idx = lower_bound(&self.zeros, pointer + nj);
                if nz_idx < self.zeros.len() {
                    let next_zero = self.zeros[nz_idx];

                    // from next_zero to the right
                    for i in 1..i_max {
                        if i >= i_max - 3 {
                            break;
                        }
                        if next_zero + i > self.max_pointer {
                            break;
                        }
                        let target = next_zero + i;
                        let idx = target as usize;
                        if idx >= self.tape.len() {
                            break;
                        }
                        let ncost = abs_diff(self.goal[start], self.tape[idx]) + i + 1 + zcost;
                        if ncost <= i_max {
                            let node = Node::new(target, ncost);
                            let tval = self.tape[idx];
                            let old_zeros = self.zeros.clone();
                            if tval == 0 {
                                self.recompute_zeros();
                            }
                            self.tape[idx] = self.goal[start];
                            let subpath = self.exhaustive(
                                cost + ncost,
                                start + 1,
                                node.pointer,
                                max_cost + next_cost,
                            );
                            self.tape[idx] = tval;
                            self.zeros = old_zeros;

                            if let Some(mut subpath) = subpath {
                                let better = min_path
                                    .as_ref()
                                    .map(|p| subpath.cost() + ncost < p.cost())
                                    .unwrap_or(true);
                                let unique_ok = !self.unique_cells || !subpath.contains(&node);
                                if better && unique_ok {
                                    subpath.add_first(node);
                                    min_path = Some(subpath);
                                }
                            }
                        }
                    }

                    // from next_zero back towards pointer
                    for i in 1..i_max {
                        if i >= i_max - 3 {
                            break;
                        }
                        let limit = (next_zero - pointer - 3) >> 1;
                        if i > limit {
                            break;
                        }
                        let target = next_zero - i;
                        if target < 0 {
                            break;
                        }
                        let idx = target as usize;
                        if idx >= self.tape.len() {
                            break;
                        }
                        let ncost = abs_diff(self.goal[start], self.tape[idx]) + i + 1 + zcost;
                        if ncost <= i_max {
                            let node = Node::new(target, ncost);
                            let tval = self.tape[idx];
                            let old_zeros = self.zeros.clone();
                            if tval == 0 {
                                self.recompute_zeros();
                            }
                            self.tape[idx] = self.goal[start];
                            let subpath = self.exhaustive(
                                cost + ncost,
                                start + 1,
                                node.pointer,
                                max_cost + next_cost,
                            );
                            self.tape[idx] = tval;
                            self.zeros = old_zeros;

                            if let Some(mut subpath) = subpath {
                                let better = min_path
                                    .as_ref()
                                    .map(|p| subpath.cost() + ncost < p.cost())
                                    .unwrap_or(true);
                                let unique_ok = !self.unique_cells || !subpath.contains(&node);
                                if better && unique_ok {
                                    subpath.add_first(node);
                                    min_path = Some(subpath);
                                }
                            }
                        }
                    }

                    // roll to next zero [.>]
                    let mut run_len = next_zero - pointer - nj;
                    while nj < 1
                        && pointer + nj > 0
                        && self.tape[(pointer + nj - 1) as usize] != 0
                        && run_len < (self.goal.len() - start) as i32
                    {
                        nj -= 1;
                        run_len += 1;
                    }
                    while run_len >= 4 && run_len as usize <= self.goal.len() - start {
                        let target = pointer + nj;
                        let idx = target as usize;
                        if idx >= self.tape.len() {
                            break;
                        }
                        let ncost = abs_diff(self.goal[start], self.tape[idx]) + nj.abs() + 1;
                        if ncost <= i_max {
                            let mut subpath = Path::new();
                            subpath.add_last(Node::new_with_rolling(target, ncost + 3));
                            let mut dcost = next_cost;
                            let mut valid = true;
                            for i in 1..run_len {
                                let cell = pointer + nj + i;
                                if cell > self.max_pointer {
                                    valid = false;
                                    break;
                                }
                                let cidx = cell as usize;
                                if cidx >= self.tape.len() {
                                    valid = false;
                                    break;
                                }
                                let delta =
                                    abs_diff(self.goal[start + i as usize], self.tape[cidx]);
                                if delta > i_max {
                                    valid = false;
                                    break;
                                }
                                subpath.add_last(Node::new_with_rolling(cell, delta));
                                dcost += if start + i as usize + 1 == self.goal.len() {
                                    0
                                } else if self.goal[start + i as usize]
                                    == self.goal[start + i as usize + 1]
                                {
                                    1
                                } else {
                                    2
                                };
                            }
                            if valid && subpath.len() as i32 == run_len {
                                let mut saved = Vec::with_capacity(run_len as usize);
                                for node in subpath.iter() {
                                    saved.push(self.tape[node.pointer as usize]);
                                }
                                for (offset, node) in subpath.iter().enumerate() {
                                    self.tape[node.pointer as usize] = self.goal[start + offset];
                                }
                                if let Some(subpath2) = self.exhaustive(
                                    cost + subpath.cost(),
                                    start + run_len as usize,
                                    next_zero,
                                    max_cost + dcost,
                                ) {
                                    let better = min_path
                                        .as_ref()
                                        .map(|p| subpath.cost() + subpath2.cost() < p.cost())
                                        .unwrap_or(true);
                                    if better {
                                        let mut combined = subpath.clone();
                                        combined.extend_back(&subpath2);
                                        min_path = Some(combined);
                                    }
                                }
                                for (node, value) in subpath.iter().zip(saved.iter()) {
                                    self.tape[node.pointer as usize] = *value;
                                }
                            }
                        }
                        nj += 1;
                        run_len -= 1;
                    }
                }
            }
        }

        min_path
    }

    fn recompute_zeros(&mut self) {
        self.zeros = (0..=self.max_pointer)
            .filter(|&i| {
                let idx = i as usize;
                idx < self.tape.len() && self.tape[idx] == 0
            })
            .collect();
    }
}
