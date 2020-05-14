
use std::collections::HashSet;

fn traverse(starting_node: (Vec<usize>, usize), n: usize, max_round: usize, classes: &Vec<usize>) -> bool {
    let mut visited: HashSet<(Vec<usize>, usize)> = HashSet::new();
    visited.insert(starting_node.clone());
    let mut stack = Vec::new();
    let neigh = neighbours(&starting_node, n, max_round, &mut visited, &classes);
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
            return true;
        }
        let new_neigh = neighbours(new_current, n, max_round, &mut visited, &classes);
        top.1 += 1;
        stack.push((new_neigh, 0));
        i += 1;
    }

}


fn neighbours(node: &(Vec<usize>, usize), n: usize, max_round: usize, visited: &mut HashSet<(Vec<usize>, usize)>, classes: &Vec<usize>) -> Vec<(Vec<usize>, usize)> {
    let mut out = Vec::new();
    let max = if node.1 == 0 {
        classes.len()
    } else {
        1 << n
    };
    for k in 0..max {
        let i = if node.1 == 0 {
            classes[k]
        } else {
            k
        };
        let mut flipped = move_it(&node.0, i);

        let mut next_round = node.1 + 1;
        if next_round == max_round {
            flipped = mix_around(&flipped, n);
            next_round = 0;
        }
        let mut conv: Vec<usize> = flipped.into_iter().collect();
        conv.sort();
        let mut already_visited = false;
        let le = conv.len();
        let mut tup = (conv, next_round);
        let hue = {
            visited.contains(&tup)
        };
        if hue {
            already_visited = true;
        } else {
            for _ in 0..n {
                for ind in 0..le {
                    tup.0[ind] = rotate(tup.0[ind], n);
                }
                tup.0.sort();
                if visited.contains(&tup) {
                    already_visited = true;
                    break;
                }
            }
        }
        if !already_visited {
            visited.insert(tup.clone());
            out.push(tup);
        }
    }
    out
}

fn eq_classes(n: usize) -> Vec<usize> {
    let mut out = Vec::new();
    for mut i in 0..(1 << n) {
        if !out.contains(&i) {
            out.push(i);
        }
        for _ in 0..n {
            i = rotate(i, n);
            if !out.contains(&i) {
                out.push(i);
            }
        }
    }
    out
}

fn rotate(k: usize, n: usize) -> usize {
    ((k << 1) & ((1 << n) - 1)) | (k >> (n-1))
}

fn mix_around(possibilities: &HashSet<usize>, n: usize) -> HashSet<usize> {
    let mut out: HashSet<usize> = HashSet::new();
    for p in possibilities {
        out.insert(*p);
        let mut p = *p;
        for _ in 0..n {
            p = ((p << 1) & ((1 << n) - 1)) | (p >> (n-1));
            out.insert(p);
        }
    }
    out
}

fn move_it(possibilities: &Vec<usize>, flip: usize) -> HashSet<usize> {
    let mut out = HashSet::new();
    for p in possibilities {
        out.insert(p ^ flip);
    }
    out.remove(&0);
    out
}

fn start(n: usize) -> Vec<usize> {
    let mut out = Vec::new();
    for i in 1..(1 << n) {
        out.push(i);
    }
    out
}
fn main() {
    for n in 1..=100 {
        for phi in 1..(n+1) {
            println!("Trying {} : {}", n, phi);
            if traverse((start(n), 0), n, phi, &eq_classes(n)) {
                println!("{} : {}", n, phi);
                break;
            }
        }
    }
}