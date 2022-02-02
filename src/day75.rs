// https://atcoder.jp/contests/typical90/tasks/typical90_z

fn dfs(
    graph: &Vec<Vec<usize>>,
    color: &mut Vec<usize>,
    u: usize,
    c: usize,
  ) {
    color[u] = c;
    for &v in &graph[u] {
        if color[v] == 0 {
            dfs(graph, color, v, 3 - c);
        }
    }
}

fn solve(n: usize, ab: Vec<(usize, usize)>) -> Vec<usize>{
    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a-1].push(b-1);
        graph[b-1].push(a-1);
    }
    let mut color = vec![0; n];
    dfs(&graph, &mut color, 0, 1);
    
    let mut g1 = vec![];
    let mut g2 = vec![];
    for i in 0..n {
        match color[i] {
        1 => g1.push(i+1),
        _ => g2.push(i+1),
        }
    }
    
    let g = if g1.len() > g2.len() { g1 } else { g2 };
    g[0..n/2].iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" ");
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        assert_eq!(vec![3, 4], solve(4, vec![(1, 2), (2, 3), (2, 4)]));
        assert_eq!(vec![1, 2, 6], solve(4, vec![(1, 2), (2, 3), (2, 4)]));
    }
}
