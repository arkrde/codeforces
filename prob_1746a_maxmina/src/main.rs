use std::io;

fn main() {
    let std_input = io::stdin();
    let mut line_iterator = std_input.lines();

    let line = line_iterator.next().unwrap().unwrap();
    let test_cases : usize = line.trim().parse().unwrap();

    for _ in 0..test_cases {
        let line = line_iterator.next().unwrap().unwrap();
        let first_line_values: Vec<usize> =
            line.split(" ")
            .map(|x| x.trim())
            .filter(|x| x.len() > 0)
            .map(|x| x.parse().unwrap())
            .collect();
        let n = first_line_values[0];
        let k = first_line_values[1];
        
        let line = line_iterator.next().unwrap().unwrap();
        let second_line_values: Vec<bool> =
            line.split(" ")
            .map(|x| x.trim())
            .filter(|x| x.len() > 0)
            .map(|x| x.parse().unwrap())
            .collect();
        if compute(n, k, &second_line_values[..]) == true {
            println!("YES");
        } else {
            println!("NO");
        }
    }
    
}


fn compute(n: usize, k: usize, values: &[bool]) -> bool {
    if k > n {
        *values.into_iter().min().unwrap()
    } else {
        let first : usize = 0;
        let last : usize = ((first + k - 1)..n).step_by(k - 1).last().unwrap();
        let mut res = *values[first..=last].into_iter().max().unwrap();
        if last < n - 1 {
            res &= *values[(last + 1)..].into_iter().min().unwrap();
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_result() {
        assert_eq!(compute(2, 2, &[false, false]), false);
        assert_eq!(compute(2, 2, &[false, true]), true);
        assert_eq!(compute(2, 2, &[true, false]), true);
        assert_eq!(compute(2, 2, &[true, true]), true);
        assert_eq!(compute(3, 2, &[false, false, false]), false);
        assert_eq!(compute(3, 3, &[false, false, false]), false);
        assert_eq!(compute(3, 2, &[false, false, true]), true);
        assert_eq!(compute(4, 3, &[false, false, false, true]), true);
    }
}