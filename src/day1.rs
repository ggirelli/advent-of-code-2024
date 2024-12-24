use std::collections::HashMap;
use std::fs;
use std::iter;

fn read_input() -> String {
    let filepath = "data/day1.txt";
    let content =
        fs::read_to_string(filepath).expect("Should have been able to read the file: {filepath}");
    content
}

fn parse_input(content: String) -> (Vec<i32>, Vec<i32>) {
    let mut col1: Vec<i32> = Vec::<i32>::new();
    let mut col2: Vec<i32> = Vec::<i32>::new();

    for line in content.split("\n") {
        if line.len() == 0 {
            break;
        }
        let split_line = line.split("   ").collect::<Vec<&str>>();
        if split_line.len() != 2 {
            panic!("Expecting exactly 2 elements per line: '{line}'");
        }

        col1.push(*&split_line[0].parse::<i32>().unwrap());
        col2.push(*&split_line[1].parse::<i32>().unwrap());
    }

    println!("Parsed {} lines.", col1.len());
    (col1, col2)
}

fn panic_vecs_diff_len<T>(col1: &Vec<T>, col2: &Vec<T>) {
    if col1.len() != col2.len() {
        panic!(
            "The provided vectors have different length: {} != {}",
            col1.len(),
            col2.len()
        );
    }
}

fn calc_col_distance(col1: &Vec<i32>, col2: &Vec<i32>) -> i32 {
    panic_vecs_diff_len(&col1, &col2);
    if col1.len() != col2.len() {
        panic!(
            "The provided columns have different length: {} != {}",
            col1.len(),
            col2.len()
        );
    }
    let mut distance: i32 = 0;
    for (val1, val2) in iter::zip(col1, col2) {
        distance = distance + (val1 - val2).abs();
    }
    distance
}

fn uniq_with_counts(col: &Vec<i32>) -> HashMap<i32, i32> {
    let mut value_counts: HashMap<i32, i32> = HashMap::<i32, i32>::new();
    for value in col {
        if value_counts.contains_key(value) {
            value_counts.insert(*value, value_counts.get(value).unwrap() + 1);
        } else {
            value_counts.insert(*value, 1);
        }
    }
    value_counts
}

fn calc_similarity_score(col1: &Vec<i32>, col2: &Vec<i32>) -> i32 {
    panic_vecs_diff_len(&col1, &col2);
    let value_counts: HashMap<i32, i32> = uniq_with_counts(col2);
    let mut similarity: i32 = 0;
    for value in col1 {
        if !value_counts.contains_key(value) {
            continue;
        }
        similarity = similarity + value * value_counts.get(value).unwrap();
    }
    similarity
}

pub fn run() {
    let mut col1: Vec<i32>;
    let mut col2: Vec<i32>;
    (col1, col2) = parse_input(read_input());
    col1.sort();
    col2.sort();

    println!("Columns distance: {}", calc_col_distance(&col1, &col2));
    println!("Similarity score: {}", calc_similarity_score(&col1, &col2));
}

#[test]
fn test_example() {
    let test_input: String = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3".to_string();

    let mut col1: Vec<i32>;
    let mut col2: Vec<i32>;
    (col1, col2) = parse_input(test_input);
    col1.sort();
    col2.sort();

    assert_eq!(calc_col_distance(&col1, &col2), 11);
    assert_eq!(calc_similarity_score(&col1, &col2), 31);
}
