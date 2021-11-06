// https://leetcode.com/problems/rotate-image

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();

    for i in 0..n {
        for j in i..n {
            let tmp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = tmp;
        }
    }

    for i in 0..n {
        for j in 0..n / 2 {
            let tmp = matrix[i][j];
            matrix[i][j] = matrix[i][n - j - 1];
            matrix[i][n - j - 1] = tmp;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::rotate;
    #[test]
    fn test_rotate() {
        let mut matrix: Vec<Vec<i32>> = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        rotate(&mut matrix);
        assert_eq!(vec![15, 13, 2, 5], matrix[0]);
        assert_eq!(vec![14, 3, 4, 1], matrix[1]);
        assert_eq!(vec![12, 6, 8, 9], matrix[2]);
        assert_eq!(vec![16, 7, 10, 11], matrix[3]);
    }
}
