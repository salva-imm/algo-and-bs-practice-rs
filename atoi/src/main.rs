fn atoi_fn(s: String) -> i32 {
    let mut x = s.trim().chars().collect::<Vec<char>>();
    let mut result = String::new();

    if x.len() != 0 && (x[0].is_ascii_digit() || x[0] == '-' || x[0] == '+') {
        let mut make_negative = 1;
        if x[0] == '-' {
            make_negative = -1;
            x.remove(0);
        } else if x[0] == '+' {
            x.remove(0);
        }
        for i in x {
            if !i.is_ascii_digit() {
                break;
            }
            result += &i.to_string();
        }
        result = result.trim_start_matches('0').parse().unwrap();
        if result.len() != 0 {
            if result.len() > 10 {
                if make_negative == -1 {
                    i32::MIN
                } else {
                    i32::MAX
                }
            } else {
                let parsed = result.parse::<i64>().unwrap() * make_negative;
                std::cmp::max(i32::MIN as i64, std::cmp::min(parsed, i32::MAX as i64)) as i32
            }
        } else {
            0
        }
    } else {
        0
    }
}


fn main() {
    println!("{:?}", atoi_fn("  0000000000012345678".to_string()));
}