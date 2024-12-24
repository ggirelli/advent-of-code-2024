use crate::day1;

struct Report {
    data: Vec<i32>,
}

impl Report {
    fn is_safe(&self) -> bool {
        let mut prev_n: &i32 = &self.data[0];
        let mut all_decreasing: bool = true;
        let mut all_increasing: bool = true;
        for n in self.data.iter().skip(1) {
            let delta = n - prev_n;
            if delta < -3 || delta > 3 {
                return false;
            }
            if n > &prev_n {
                all_decreasing = false;
            } else if n < &prev_n {
                all_increasing = false;
            } else {
                return false;
            }
            prev_n = n;

            if !all_decreasing && !all_increasing {
                break;
            }
        }
        all_increasing || all_decreasing
    }

    fn tolerant_is_safe(&self) -> bool {
        if !self.is_safe() {
            for i in 0..self.data.len() {
                let mut data = self.data.to_vec();
                data.remove(i);
                let report: Report = Report { data };
                if report.is_safe() {
                    return true;
                }
            }
            return false;
        } else {
            return true;
        }
    }
}

fn parse_content(content: String) -> Vec<Report> {
    let mut report_list: Vec<Report> = Vec::<Report>::new();
    for line in content.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let mut data: Vec<i32> = Vec::<i32>::new();
        for n in line.split(" ") {
            data.push(n.parse::<i32>().unwrap());
        }
        report_list.push(Report { data });
    }
    report_list
}

fn count_safe_reports(report_list: &Vec<Report>) -> i32 {
    let mut safe_reports_count: i32 = 0;
    for report in report_list {
        if report.is_safe() {
            safe_reports_count += 1;
        }
    }
    safe_reports_count
}

fn tolerant_count_safe_reports(report_list: &Vec<Report>) -> i32 {
    let mut safe_reports_count: i32 = 0;
    for report in report_list {
        if report.tolerant_is_safe() {
            safe_reports_count += 1;
        }
    }
    safe_reports_count
}
pub fn run() {
    let report_list: Vec<Report> = parse_content(day1::read_input("data/day2.txt"));

    println!("Safe reports count: {}", count_safe_reports(&report_list));
    println!(
        "Safe reports count, with tolerant module: {}",
        tolerant_count_safe_reports(&report_list)
    );
}

#[test]
fn test_example() {
    let content: String = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
        .to_string();

    let report_list: Vec<Report> = parse_content(content);

    assert!(&report_list[0].is_safe());
    assert!(!&report_list[1].is_safe());
    assert!(!&report_list[2].is_safe());
    assert!(!&report_list[3].is_safe());
    assert!(!&report_list[4].is_safe());
    assert!(&report_list[5].is_safe());

    assert_eq!(count_safe_reports(&report_list), 2);

    assert!(&report_list[0].tolerant_is_safe());
    assert!(!&report_list[1].tolerant_is_safe());
    assert!(!&report_list[2].tolerant_is_safe());
    assert!(&report_list[3].tolerant_is_safe());
    assert!(&report_list[4].tolerant_is_safe());
    assert!(&report_list[5].tolerant_is_safe());

    assert_eq!(tolerant_count_safe_reports(&report_list), 4);
}
