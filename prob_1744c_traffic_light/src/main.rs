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
        let words: Vec<&str> = next_line
            .split(" ")
            .map(|word| word.trim())
            .filter(|word| word.len() > 0)
            .collect();
        let _n: usize = words[0].parse().unwrap();
        let current_state = words[1].chars().nth(0).unwrap();
        next_line = line_iterator
            .next()
            .unwrap()
            .expect("Error parsing third line");
        println!("{}", compute(&next_line[..], current_state));
    }
}

fn compute(seq: &str, current_state: char) -> usize {
    let mut max_distance: usize = 0;
    if current_state == 'g' {
        return max_distance;
    } else {
        let full_seq = String::from(seq) + seq;
        let mut start_idx = 0;
        while start_idx < seq.len() {
            let mut start_iter = full_seq[start_idx..]
                .chars()
                .enumerate()
                .skip_while(|(_, c)| c != &current_state);
            start_idx += start_iter.next().unwrap().0;
            let mut go_iter = full_seq[start_idx..]
                .chars()
                .enumerate()
                .skip_while(|(_, c)| c != &'g');
            match go_iter.next() {
                Some((dist, _)) => {
                    if max_distance < dist {
                        max_distance = dist;
                    }
                    start_idx += dist + 1;
                }
                None => break,
            }
        }
        max_distance
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_func() {
        assert_eq!(compute("rggry", 'r'), 3);
        assert_eq!(compute("g", 'g'), 0);
        assert_eq!(compute("rrg", 'r'), 2);
        assert_eq!(compute("yrrgy", 'y'), 4);
        assert_eq!(compute("rgrgyrg", 'r'), 1);
        assert_eq!(compute("rrrgyyygy", 'y'), 4);
    }
}
