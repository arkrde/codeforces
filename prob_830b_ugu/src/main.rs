use std::io;
fn main() {
    let mut line_iterator = io::stdin().lines();
    let mut next_line = line_iterator
        .next()
        .unwrap()
        .expect("Error parsing first line");
    let num_test_cases: usize = next_line.trim().parse().unwrap();
    for _ in 0..num_test_cases {
        next_line = line_iterator
            .next()
            .unwrap()
            .expect("Error parsing second line");
        let _n: usize = next_line.trim().parse().unwrap();
        next_line = line_iterator
            .next()
            .unwrap()
            .expect("Error parsing third line");
        let seq: Vec<usize> = next_line
            .chars()
            .map(|w| w.to_string().parse().unwrap())
            .collect();
        println!("{}", solution(&seq));
    }
}

fn solution(seq: &Vec<usize>) -> usize {
    if seq == &vec![0] {
        0
    } else if seq == &vec![1] {
        0
    } else {
        seq[0..].iter().zip(seq[1..].iter()).fold(0, |ops, (a, b)| {
            if (a + ops) % 2 == 1 && (b + ops) % 2 == 0 {
                ops + 1
            } else {
                ops
            }
        })
        //let mut ops: usize = 0;
        //for (a, b) in seq[0..].iter().zip(seq[1..].iter()) {
        //if (a + ops) % 2 == 1 && (b + ops) % 2 == 0 {
        //ops += 1;
        //}
        //}
        //ops
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(solution(&vec![0]), 0);
        assert_eq!(solution(&vec![1]), 0);
        assert_eq!(solution(&vec![1, 0]), 1);
        assert_eq!(solution(&vec![1, 0, 1]), 2);
        assert_eq!(solution(&vec![1, 1, 0, 0]), 1);
        assert_eq!(solution(&vec![1, 1, 0, 0, 1]), 2);
        assert_eq!(solution(&vec![1, 0, 0, 0, 1, 0]), 3);
        assert_eq!(solution(&vec![0, 0, 0, 0, 1, 1, 0, 0, 0, 0]), 1);
        assert_eq!(solution(&vec![0, 1, 0, 1, 0, 1, 0]), 5);
        assert_eq!(solution(&vec![0, 0, 0, 0, 0]), 0);
        assert_eq!(solution(&vec![1, 1, 1, 1, 1]), 0);
        assert_eq!(solution(&vec![0, 0, 0, 0, 0, 1]), 0);
    }
}
