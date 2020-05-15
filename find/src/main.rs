
use std::collections::HashSet;
use std::collections::HashMap;

const N: usize = 6;
const PHI: usize = 5;

fn traverse(starting_node: (Vec<u8>, u8), classes: &Vec<usize>) -> bool {
    let mut visited: HashMap<(u64, u8), Option<((u64, u8), u8)>> = HashMap::new();
    let nnn = to_number(&starting_node.0);
    visited.insert((nnn, starting_node.1), None);
    let mut stack = Vec::new();
    let neigh = neighbours(&starting_node, &mut visited, &classes);
    stack.push((neigh, 0));
    let mut i = 0;
    loop {
        let top = &mut stack[i];
        if top.1 == top.0.len() {
            stack.pop();
            if i == 0 {
                return false;
            }
            i -= 1;
            continue;
        }

        let new_current = &top.0[top.1];
        if new_current.0.len() == 0 {
            find_history(visited, new_current.clone());
            return true;
        }
        let new_neigh = neighbours(new_current, &mut visited, &classes);
        top.1 += 1;
        stack.push((new_neigh, 0));
        i += 1;
    }
}

fn find_history(visited: HashMap<(u64, u8), Option<((u64, u8), u8)>>, end_node: (Vec<u8>, u8)) {
    let conv = to_number(&end_node.0);
    let mut conv_tup = (conv, end_node.1);
    let (mut node, mut mov) = visited.get(&conv_tup).unwrap().unwrap();
    println!("SO YOU WANT TO ESCAPE FROM YOUR LAIR? WELL JUST FOLLOW THESE INSTRUCTIONS");
    let mut out = Vec::new();
    out.push(mov);

    loop {
        let tup = *visited.get(&node).unwrap();
        match tup {
            Some((a, b)) => {
                node = a;
                mov = b;
            },
            None => break,
        }
        out.push(mov)
    }
    out.reverse();
    for o in out {
        println!("{:06b}", o);
    }
}



fn neighbours(node: &(Vec<u8>, u8), visited: &mut HashMap<(u64, u8), Option<((u64, u8), u8)>>, classes: &Vec<usize>) -> Vec<(Vec<u8>, u8)> {
    let mut out: Vec<(Vec<u8>, u8)> = Vec::new();
    let max = if node.1 == 0 {
        classes.len()
    } else {
        1 << N
    };
    for k in 0..max {
        let i = if node.1 == 0 {
            classes[k] 
        } else {
            k
        };
        let i = i as u8;
        let mut flipped = move_it(&node.0, i);

        let mut next_round = node.1 + 1;
        if next_round == (PHI as u8) {
            flipped = mix_around(&flipped);
            next_round = 0;
        }
        if !perms_visited(&mut flipped, next_round, visited) {
            let nnn = to_number(&flipped);
            let orig_nnn = to_number(&node.0);
            visited.insert((nnn, next_round), Some(((orig_nnn, node.1), i)));
            flipped.sort();
            out.push((flipped, next_round));
        }
    }
    out
}

fn swapper(p: usize, q: usize, values: &mut Vec<u8>) {
    for j in 0..values.len() {
        let v = values[j];

        // Stupid bit-fiddling
        let p_value = (v & (1 << p)) >> p;
        let q_value = (v & (1 << q)) >> q;
        let pq_mask = !((1 << p) | (1 << q));
        values[j] = (v & pq_mask) | (p_value << q) | (q_value << p);
    }
}

// This function modifies 'orig' but returns it unchanged when the return value is true.
fn perms_visited(orig: &mut Vec<u8>, next_round: u8, visited: &HashMap<(u64, u8), Option<((u64, u8), u8)>>) -> bool {
    let n = N;
    let nnn = to_number(&orig);
    if visited.contains_key(&(nnn, next_round)) {
        return true;
    }
    let mut perm: Vec<u8> = (0..(n as u8)).collect();
    let mut dirs: Vec<i8> = vec![-1;n];
    dirs[0] = 0;
    loop {
        let mut i: usize = 0;
        let mut changed = false;
        let mut e = 0;
        for j in 0..n {
            if (dirs[j] != 0) && (perm[j] > e) {
                e = perm[j];
                i = j;
                changed = true;
            }
        }
        
        if !changed {
            swapper(0, 1, orig);
            return false;
        }
        let k: usize = ((i as i8) + dirs[i as usize]) as usize;

        dirs.swap(i, k);
        perm.swap(i, k);
        swapper(i, k, orig);

        if (k == 0) || (k == n-1) || (perm[(k as i64 + (dirs[k] as i64)) as usize] > e) {
            dirs[k] = 0;
        }

        for j in 0..n {
            if perm[j] > e {
                dirs[j] = if j < k {
                    1
                } else {
                    -1
                };
            }
        }
        let nnn = to_number(&orig);
        if visited.contains_key(&(nnn, next_round)) {
            return true;
        }
    }
}

fn to_number(vs: &Vec<u8>) -> u64 {
    let mut out = 0;
    for &v in vs {
        assert!(v < 64);
        out |= 1 << v;
    }
    out
}

fn eq_classes() -> Vec<usize> {
    let mut out = Vec::new();
    for mut i in 0..(1 << N) {
        if !out.contains(&i) {
            out.push(i);
        }
        for _ in 0..N {
            i = rotate(i);
            if !out.contains(&i) {
                out.push(i);
            }
        }
    }
    out
}

fn rotate(k: usize) -> usize {
    ((k << 1) & ((1 << N) - 1)) | (k >> (N-1))
}

fn mix_around(possibilities: &Vec<u8>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for p in possibilities {
        let mut p = *p;
        for _ in 0..N {
            p = ((p << 1) & ((1 << N) - 1)) | (p >> (N-1));
            out.push(p);
        }
    }
    out.sort();
    out.dedup();
    out
}

fn move_it(possibilities: &Vec<u8>, flip: u8) -> Vec<u8> {
    let mut out = Vec::new();
    for p in possibilities {
        let flipped = p ^ flip;
        if flipped != 0 {
            out.push(flipped);
        }
    }
    out
}

fn start(n: usize) -> Vec<u8> {
    let mut out = Vec::new();
    for i in 1..(1 << n) {
        out.push(i);
    }
    out
}
fn main() {
    dbg!(N, PHI);
    let a = traverse((start(N), 0), &eq_classes());
    dbg!(a);
}
