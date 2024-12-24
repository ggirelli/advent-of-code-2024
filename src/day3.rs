use crate::day1;

struct Operation {
    a: i32,
    b: i32,
}

impl Operation {
    pub fn result(&self) -> i32 {
        self.a * self.b
    }
}

fn parse_input(content: &String) -> Vec<Operation> {
    let mut operations_list: Vec<Operation> = Vec::<Operation>::new();

    let accepted_chars: String = "mul(),0123456789".to_string();

    let mut operation_str: String = "".to_string();
    let mut building_operator1: bool = false;
    let mut operator1_str: String = "".to_string();
    let mut building_operator2: bool = false;
    let mut operator2_str: String = "".to_string();
    for c in content.chars() {
        if !accepted_chars.contains(c) {
            operation_str = "".to_string();
            building_operator1 = false;
            operator1_str = "".to_string();
            building_operator2 = false;
            operator2_str = "".to_string();
            continue;
        }

        if c == '(' {
            if operation_str != "mul" {
                operation_str = "".to_string();
                building_operator1 = false;
                operator1_str = "".to_string();
                building_operator2 = false;
                operator2_str = "".to_string();
                continue;
            }
            building_operator1 = true;
            continue;
        } else if c == ',' {
            if operator1_str == "" {
                operation_str = "".to_string();
                building_operator1 = false;
                operator1_str = "".to_string();
                building_operator2 = false;
                operator2_str = "".to_string();
                continue;
            }
            building_operator1 = false;
            building_operator2 = true;
            continue;
        } else if c == ')' {
            if operator1_str != "" && operator2_str != "" {
                operations_list.push(Operation {
                    a: operator1_str.parse::<i32>().unwrap(),
                    b: operator2_str.parse::<i32>().unwrap(),
                });
            }
            operation_str = "".to_string();
            building_operator1 = false;
            operator1_str = "".to_string();
            building_operator2 = false;
            operator2_str = "".to_string();
            continue;
        }

        if building_operator1 {
            operator1_str.push(c);
        } else if building_operator2 {
            operator2_str.push(c);
        }
        operation_str.push(c);
    }
    operations_list
}

fn sum_ops_results(operations_list: &Vec<Operation>) -> i32 {
    let mut result: i32 = 0;
    for op in operations_list {
        result += op.result();
    }
    result
}

struct Multiplication {
    a: i32,
    b: i32,
    enabled: bool,
}

impl Multiplication {
    pub fn result(&self) -> i32 {
        self.a * self.b
    }
}

fn parse_input_with_dos(content: &String) -> Vec<Multiplication> {
    let mut operations_list: Vec<Multiplication> = Vec::<Multiplication>::new();

    let accepted_chars: String = "don'tmul(),0123456789".to_string();
    let operators_chars: String = "0123456789,".to_string();
    let mut operation_type: &str = "";
    let mut str_to_parse: String = "".to_string();
    let mut enabled: bool = true;
    for c in content.chars() {
        if !accepted_chars.contains(c) {
            str_to_parse = "".to_string();
            operation_type = "";
            continue;
        } else if c == '(' {
            if str_to_parse.ends_with("don't") {
                operation_type = "dont";
            } else if str_to_parse.ends_with("do") {
                operation_type = "do";
            } else if str_to_parse.ends_with("mul") {
                operation_type = "mul";
            } else {
                operation_type = "";
            }
            str_to_parse = "".to_string();
        } else if c == ')' {
            match operation_type {
                "do" => {
                    if str_to_parse == "" {
                        enabled = true;
                    }
                }
                "dont" => {
                    if str_to_parse == "" {
                        enabled = false;
                    }
                }
                "mul" => {
                    let ab: Vec<String> = str_to_parse.split(",").map(|s| s.to_string()).collect();
                    if ab.len() == 2 {
                        operations_list.push(Multiplication {
                            a: ab[0].parse::<i32>().unwrap(),
                            b: ab[1].parse::<i32>().unwrap(),
                            enabled: enabled,
                        });
                    }
                }
                _ => {}
            }
            operation_type = "";
            str_to_parse = "".to_string();
        } else if operation_type != "" && !operators_chars.contains(c) {
            str_to_parse = "".to_string();
            operation_type = "";
        } else {
            str_to_parse.push(c);
        }
    }

    operations_list
}

fn sum_mul_results(operations_list: &Vec<Multiplication>) -> i32 {
    let mut result: i32 = 0;
    for op in operations_list {
        if !op.enabled {
            continue;
        }
        result += op.result();
    }
    result
}

pub fn run() {
    let content: String = day1::read_input("data/day3.txt");
    let operations_list: Vec<Operation> = parse_input(&content);
    println!("Operations result: {}", sum_ops_results(&operations_list));

    let multiplication_list: Vec<Multiplication> = parse_input_with_dos(&content);
    println!(
        "Operations result (with dos): {}",
        sum_mul_results(&multiplication_list)
    );
}

#[test]
fn test_example() {
    let content: String =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
    let operations_list: Vec<Operation> = parse_input(&content);
    assert_eq!(sum_ops_results(&operations_list), 161);

    let content2: String =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
    let multiplication_list: Vec<Multiplication> = parse_input_with_dos(&content2);
    assert_eq!(sum_mul_results(&multiplication_list), 48);
}
