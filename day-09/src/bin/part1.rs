use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn is_seq_zero(seq: &Vec<i32>) -> bool {
    let mut res = true;
    for v in seq {
        if v.to_owned() != 0 {
            res = false;
        }
    }

    res
}

fn get_diff_seq(seq: &Vec<i32>) -> Vec<i32> {
    let mut diff_seq: Vec<i32> = vec![];

    for i in 1..seq.len() {
        diff_seq.push(seq[i] - seq[i - 1]);
    }

    diff_seq
}

fn main() {
    let lines = read_lines("input1.txt");
    let mut base_sequences: Vec<Vec<i32>> = vec![];
    let mut end_seq_list: Vec<Vec<Vec<i32>>> = vec![];
    let mut extrapolated_values: Vec<i32> = vec![];

    for line in lines {
        let mut s: Vec<i32> = vec![];
        let l: Vec<&str> = line.trim().split(" ").collect();
        for v in l {
            s.push(v.parse().unwrap());
        }
        base_sequences.push(s);
    }

    for base_seq in base_sequences {
        let mut seq_list: Vec<Vec<i32>> = vec![];
        let mut seq = base_seq;
        seq_list.push(seq.clone());
        loop {
            seq = get_diff_seq(&seq);
            seq_list.push(seq.clone());
            if is_seq_zero(&seq) {
                break;
            }
        }
        end_seq_list.push(seq_list);
    }

    for mut seq_tree in end_seq_list {
        seq_tree.reverse();
        let mut last_seq: Vec<i32> = vec![];
        for i in 0..seq_tree.len() {
            let mut seq = &mut seq_tree[i];
            if i == 0 {
                seq.push(0);
            } else {
                seq.push(seq.last().unwrap() + last_seq.last().unwrap());
            }
            last_seq = seq.clone();
        }
        extrapolated_values.push(*seq_tree.last().unwrap().last().unwrap());
    }

    let mut sum = 0;
    for v in extrapolated_values {
        sum += v;
    }

    println!("sum {}", sum);
}
