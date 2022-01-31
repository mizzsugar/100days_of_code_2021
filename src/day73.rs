// https://atcoder.jp/contests/typical90/tasks/typical90_c

fn solve(n: usize, ab: Vec<(usize, usize)>) -> usize {
    let mut graph = vec![vec![]; n as usize];
    for (a, b) in ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    graph.iter_mut().for_each(|x| x.sort_unstable());
    let diameter = find_end_point(&graph, find_end_point(&graph, 0).0).1;
 
    diameter + 1
}

fn find_end_point(graph: &Vec<Vec<usize>>, s: usize) -> (usize, usize) {
    let mut ret = (s, 0);
    let mut deq = VecDeque::new();
    deq.push_back((s, 0, s));
    while !deq.is_empty() {
        let (curr, dist, par) = deq.pop_front().unwrap();
        ret = (curr, dist);
        for &next in graph[curr].iter() {
            if next == par {
                continue;
            }
            deq.push_back((next, dist + 1, curr));
        }
    }
    ret
}


#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(3, solve(3, vec![(1, 2), (2, 3)]));
        assert_eq!(4, solve(5, vec![(1, 2), (2, 3), (3, 4), (3, 5)]));
    }
}
