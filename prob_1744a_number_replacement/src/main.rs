use std::collections::{HashMap, HashSet};
use std::io;

fn main() {
    let std_input = io::stdin();
    let mut line_iterator = std_input.lines();

    let line = line_iterator.next().unwrap().unwrap();
    let test_cases: usize = line.trim().parse().unwrap();

    for _ in 0..test_cases {
        let line = line_iterator.next().unwrap().unwrap();
        let _n: usize = line.trim().parse().unwrap();

        let line = line_iterator.next().unwrap().unwrap();
        let num_values: Vec<i32> = line
            .split(" ")
            .map(|x| x.trim())
            .filter(|x| x.len() > 0)
            .map(|x| x.parse().unwrap())
            .collect();

        let line = line_iterator.next().unwrap().unwrap();
        let char_values = String::from(line.trim());
        if compute(&num_values[..], char_values.as_bytes()) == true {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

fn compute(num_array: &[i32], char_array: &[u8]) -> bool {
    let mut values: Vec<(i32, u8)> = Vec::new();
    for (a, b) in num_array.iter().zip(char_array.iter()) {
        values.push((*a, *b));
    }
    let unique_values: HashSet<(i32, u8)> = values.into_iter().collect();
    let mut value_map: HashMap<i32, usize> = HashMap::new();
    for (a, _) in unique_values {
        match value_map.get_mut(&a) {
            Some(count) => *count += 1,
            _ => {
                value_map.insert(a, 1);
            }
        }
    }
    let max_count = value_map.into_values().max().unwrap();
    max_count == 1
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_func() {
        assert_eq!(compute(&[2, 3, 2, 4, 1], "cacta".as_bytes()), true);
        assert_eq!(compute(&[50], "a".as_bytes()), true);
        assert_eq!(compute(&[11, 22], "ab".as_bytes()), true);
        assert_eq!(compute(&[1, 2, 2, 1], "aaab".as_bytes()), false);
        assert_eq!(compute(&[1, 2, 3, 2, 1], "aaaaa".as_bytes()), true);
        assert_eq!(compute(&[1, 20, 2, 9, 3, 8], "azzfdb".as_bytes()), true);
    }
}
