use std::cmp::{max, min};

use anyhow::Result;

use crate::options::Options;
use crate::solver::Solver;
use crate::util::{add_byte, negate_byte, to_iso_8859_1_bytes, unescape_regex_like};

const MODINV256: [i32; 40] = [
    0, 1, 0, 171, 0, 205, 0, 183, 0, 57, 0, 163, 0, 197, 0, 239, 0, 241, 0, 27, 0, 61, 0, 167, 0,
    41, 0, 19, 0, 53, 0, 223, 0, 225, 0, 139, 0, 173, 0, 151,
];

pub struct Cruncher {
    min_tape: i32,
    max_tape: i32,
    max_node_cost: i32,
    _max_loops: i32,
    min_slen: i32,
    max_slen: i32,
    min_clen: i32,
    max_clen: i32,
    goal: Vec<u8>,
    limit: i32,
    rolling_limit: bool,
    unique_cells: bool,
}

impl Cruncher {
    pub fn new(options: &Options) -> Result<Self> {
        let decoded = unescape_regex_like(&options.text)?;
        let goal = to_iso_8859_1_bytes(&decoded)?;

        let (limit, rolling_limit) = if let Some(limit) = options.limit {
            (limit, options.rolling_limit)
        } else {
            let mut diff = 0;
            let mut last = 0u8;
            for &b in &goal {
                diff += crate::util::abs_diff(b, last);
                last = b;
            }
            ((diff / 3) + goal.len() as i32 + 20, true)
        };

        Ok(Self {
            min_tape: options.min_tape,
            max_tape: options.max_tape,
            max_node_cost: options.max_node_cost,
            _max_loops: options.max_loops,
            min_slen: options.min_slen,
            max_slen: options.max_slen.unwrap_or(i32::MAX),
            min_clen: options.min_clen,
            max_clen: options.max_clen.unwrap_or(i32::MAX),
            goal,
            limit,
            rolling_limit,
            unique_cells: options.unique_cells,
        })
    }

    pub fn crunch(&mut self, len: i32) {
        println!("init-len: {}; limit: {}", len, self.limit);

        let s_min = max(self.min_slen, 1);
        let s_max = min(self.max_slen, len - 12);
        if s_min > s_max {
            return;
        }

        for slen in s_min..=s_max {
            for s in s_list_gen(slen) {
                let c_min = max(self.min_clen, 3);
                let c_max = min(self.max_clen, len - slen - 9);
                if c_min > c_max {
                    continue;
                }

                for clen in c_min..=c_max {
                    for c in c_list_gen(clen) {
                        let remaining = len - slen - clen - 9;
                        if remaining < 0 {
                            continue;
                        }

                        for klen in 0..=remaining {
                            for k in k_list_gen(klen) {
                                let j_remaining = len - slen - clen - klen - 7;
                                if j_remaining < 2 {
                                    continue;
                                }

                                for jlen in 2..=j_remaining {
                                    for j in j_list_gen(jlen) {
                                        let hlen = len - slen - clen - klen - jlen - 7;
                                        let h_candidates = if hlen > 0 {
                                            vec![-hlen, hlen]
                                        } else {
                                            vec![hlen]
                                        };

                                        for h in h_candidates {
                                            if let Some((pntr, tape)) =
                                                self.fill_tape(&s, &c, k[0], k[1], j[0], j[1], h)
                                            {
                                                let max_pntr = pntr + c.len() as i32 + 1;
                                                if pntr > 0
                                                    && max_pntr >= self.min_tape
                                                    && max_pntr <= self.max_tape
                                                {
                                                    self.try_solve(
                                                        len, pntr, max_pntr, &s, &c, &k, &j, h,
                                                        tape,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn try_solve(
        &mut self,
        len: i32,
        pntr: i32,
        max_pntr: i32,
        s: &[i32],
        c: &[i32],
        k: &[i32; 2],
        j: &[i32; 2],
        h: i32,
        mut tape: Vec<u8>,
    ) {
        let mut solver1 = Solver::new(
            &self.goal,
            tape.clone(),
            pntr,
            max_pntr,
            self.max_node_cost,
            self.unique_cells,
        );
        let solution1 = solver1.solve(self.limit - len);

        if let Some(ref path1) = solution1 {
            self.report_solution(len, pntr, max_pntr, s, c, k, j, h, &tape, path1);
            if self.rolling_limit {
                self.limit = path1.cost() + len;
            }
        }

        if c.len() > 1 {
            for i in 1..=c.len() {
                let idx = (pntr + i as i32) as usize;
                if idx < tape.len() {
                    tape[idx] = negate_byte(tape[idx]);
                }
            }

            let mut solver2 = Solver::new(
                &self.goal,
                tape.clone(),
                pntr,
                max_pntr,
                self.max_node_cost,
                self.unique_cells,
            );
            if let Some(path2) = solver2.solve(self.limit - len) {
                let better = solution1
                    .as_ref()
                    .map(|p| path2.cost() < p.cost())
                    .unwrap_or(true);
                if better {
                    let sneg: Vec<i32> = s.iter().map(|v| -*v).collect();
                    let kneg = [-k[0], -k[1]];
                    let jneg = [-j[0], j[1]];
                    self.report_solution(
                        len, pntr, max_pntr, &sneg, c, &kneg, &jneg, -h, &tape, &path2,
                    );
                    if self.rolling_limit {
                        self.limit = path2.cost() + len;
                    }
                }
            }
        }
    }

    fn report_solution(
        &self,
        len: i32,
        pntr: i32,
        max_pntr: i32,
        s: &[i32],
        c: &[i32],
        k: &[i32; 2],
        j: &[i32; 2],
        h: i32,
        tape: &[u8],
        path: &crate::path::Path,
    ) {
        let program = to_bf_string(s, c, k, j, h);
        println!("{}: {}", path.cost() + len, program);
        println!("{}, {}", pntr, path);

        let min_pointer = path.iter().map(|node| node.pointer).min().unwrap_or(pntr);
        let start = min_pointer.max(0) as usize;
        let count = (max_pntr - min_pointer).max(0) as usize;
        let end = tape.len().min(start.saturating_add(count));
        let cells = tape[start..end]
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        println!("{}", cells);
    }

    fn fill_tape(
        &self,
        s: &[i32],
        c: &[i32],
        k0: i32,
        k1: i32,
        j0: i32,
        j1: i32,
        h: i32,
    ) -> Option<(i32, Vec<u8>)> {
        let extra = s.len() as i32;
        if extra < 0 {
            return None;
        }
        let size = self.max_tape.checked_add(extra)? as usize;
        let mut tape = vec![0u8; size.max((self.max_tape + 2) as usize)];

        for (idx, value) in s.iter().enumerate() {
            let pos = idx + 2;
            if pos >= tape.len() {
                return None;
            }
            tape[pos] = (*value as i32) as u8;
        }

        if j1 == 0 {
            return None;
        }

        let mut lsb = j1 & -j1;
        let mask = lsb - 1;
        let mut shift = 0;
        while (lsb & 1) == 0 {
            lsb >>= 1;
            shift += 1;
        }
        let inv_idx = (j1 >> shift) as usize;
        if inv_idx >= MODINV256.len() {
            return None;
        }
        let m = MODINV256[inv_idx];
        if m == 0 {
            return None;
        }

        let mut pntr = 2i32;
        let stop = self.max_tape - c.len() as i32;
        if stop <= pntr {
            return None;
        }

        while pntr < stop {
            let idx = pntr as usize;
            if idx >= tape.len() {
                return None;
            }
            if tape[idx] == 0 {
                break;
            }

            tape[idx] = add_byte(tape[idx], k0);
            if tape[idx] != 0 {
                if ((tape[idx] as i32) & mask) != 0 {
                    return None;
                }
                let tmp = ((tape[idx] as i32) >> shift) * m;
                for (offset, coeff) in c.iter().enumerate() {
                    let t_idx = pntr + offset as i32 + 1;
                    if t_idx < 0 {
                        return None;
                    }
                    let t_idx = t_idx as usize;
                    if t_idx >= tape.len() {
                        return None;
                    }
                    tape[t_idx] = add_byte(tape[t_idx], tmp * *coeff);
                }
                let left_idx = pntr - 1;
                if left_idx < 0 {
                    return None;
                }
                let left_idx = left_idx as usize;
                if left_idx >= tape.len() {
                    return None;
                }
                tape[left_idx] = add_byte(tape[left_idx], tmp * j0);
            }

            tape[idx] = (h as i32) as u8;
            pntr += 1;
            let idx = pntr as usize;
            if idx >= tape.len() {
                return None;
            }
            tape[idx] = add_byte(tape[idx], k1);
        }

        if pntr < stop {
            Some((pntr, tape))
        } else {
            None
        }
    }
}

fn to_bf_string(s: &[i32], c: &[i32], k: &[i32; 2], j: &[i32; 2], h: i32) -> String {
    let mut sb = String::new();
    let mut sdelim = '[';
    for &sterm in s {
        let mut prefix = String::new();
        let sign = if sterm < 0 { '-' } else { '+' };
        for _ in 0..sterm.abs() {
            prefix.push(sign);
        }
        prefix.push(sdelim);
        sb = format!("{}{}", prefix, sb);
        sdelim = '<';
    }

    append_repeated(&mut sb, if k[0] < 0 { '-' } else { '+' }, k[0].abs());
    sb.push_str("[<");
    append_repeated(&mut sb, if j[0] < 0 { '-' } else { '+' }, j[0].abs());
    sb.push('>');
    append_repeated(&mut sb, '-', j[1].abs());
    for &cterm in c {
        sb.push('>');
        append_repeated(&mut sb, if cterm < 0 { '-' } else { '+' }, cterm.abs());
    }
    append_repeated(&mut sb, '<', c.len() as i32);
    sb.push(']');
    append_repeated(&mut sb, if h < 0 { '-' } else { '+' }, h.abs());
    sb.push('>');
    append_repeated(&mut sb, if k[1] < 0 { '-' } else { '+' }, k[1].abs());
    sb.push(']');

    sb
}

fn append_repeated(sb: &mut String, ch: char, count: i32) {
    for _ in 0..count {
        sb.push(ch);
    }
}

fn s_list_gen(len: i32) -> Vec<Vec<i32>> {
    fn dfs(len: i32, first: bool, current: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
        if len < 1 {
            out.push(current.clone());
            return;
        }
        for i in -len..=len {
            if (first && i == 0) || (i.abs() == len - 1) {
                continue;
            }
            current.push(i);
            dfs(len - i.abs() - 1, false, current, out);
            current.pop();
        }
    }

    let mut result = Vec::new();
    let mut current = Vec::new();
    dfs(len, true, &mut current, &mut result);
    result
}

fn c_list_gen(len: i32) -> Vec<Vec<i32>> {
    fn dfs(len: i32, first: bool, current: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
        if len < 1 {
            out.push(current.clone());
            return;
        }
        let j = if first { 1 } else { 2 };
        for i in (j - len)..=(len - j) {
            if i == 0 && len < 3 && !first {
                continue;
            }
            current.push(i);
            dfs(len - i.abs() - j, false, current, out);
            current.pop();
        }
    }

    let mut result = Vec::new();
    let mut current = Vec::new();
    dfs(len, true, &mut current, &mut result);
    result
}

fn k_list_gen(len: i32) -> Vec<[i32; 2]> {
    if len == 0 {
        return vec![[0, 0]];
    }
    let mut result = Vec::new();
    result.push([-len, 0]);
    for i in (1 - len)..len {
        let k1 = len - i.abs();
        result.push([i, k1]);
        result.push([i, -k1]);
    }
    result.push([len, 0]);
    result
}

fn j_list_gen(len: i32) -> Vec<[i32; 2]> {
    let mut result = Vec::new();
    for i in 1..len {
        result.push([len - i, i]);
    }
    result
}
