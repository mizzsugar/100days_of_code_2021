// https://github.com/E869120/kyopro_educational_90/blob/main/problem-txt/002.txt
// WIP

fn hantei(s: &String) -> bool {
    let mut dep: i8 = 0;
    let to_char: Vec<char> = s.chars().collect();
    for i in 0..s.len() {
        if to_char[i] == '(' {
            dep += 1;
        }
        if to_char[i] == ')' {
            dep -= 1;
        }
        if dep < 0 {
            return false
        }
    }
    return dep == 0
}

fn generate(n: u8, i: u8, j: u8) -> String{
    let mut candidate = String::new();
    for j in (0..n-1).rev() {
        if i & 1 << j == 0 {
            candidate.push('(');
        } else {
            candidate.push(')');
        }
    }
    candidate
}

fn solve(n: u8) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for i in 0..1<<n {
        let j = n - 1;
        let candidate = generate(n, i, j);
        if hantei(&candidate) {
            result.push(candidate);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_solve() {
        let actual = solve(2);
        let expected = String::from("()");
        assert!(actual.contains(&expected));
    }
}
