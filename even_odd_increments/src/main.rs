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
        let _n = first_line_values[0];
        let q = first_line_values[1];
        
        let line = line_iterator.next().unwrap().unwrap();
        let mut num_values: Vec<i64> =
            line.split(" ")
            .map(|x| x.trim())
            .filter(|x| x.len() > 0)
            .map(|x| x.parse().unwrap())
            .collect();
       
        for _ in 0..q {
            let line = line_iterator.next().unwrap().unwrap();
            let query_lines: Vec<i64> =
                line.split(" ")
                .map(|x| x.trim())
                .filter(|x| x.len() > 0)
                .map(|x| x.parse().unwrap())
                .collect();
            let query_type = query_lines[0];
            let query_value = query_lines[1];
            if query_type == 0 {
                compute_even_query(&mut num_values, query_value);
            } else {
                compute_odd_query(&mut num_values, query_value);
            }
            println!("{}", num_values.iter().sum::<i64>());
        }
    }
}


fn compute_even_query(values: &mut [i64], change: i64) {
    for e in values {
        if *e % 2 == 0 {
            *e += change;
        }
    }
}

fn compute_odd_query(values: &mut [i64], change: i64) {
    for e in values {
        if *e % 2 == 1 {
            *e += change;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_even_query() {
        let mut values : Vec<i64> = vec![1, 2, 4];
        compute_even_query(&mut values, 2);
        assert_eq!(values, vec![1, 4, 6]);
    }
    #[test]
    fn test_odd_query() {
        let mut values : Vec<i64> = vec![1, 4, 6];
        compute_odd_query(&mut values, 3);
        assert_eq!(values, vec![4, 4, 6]);
    }
}