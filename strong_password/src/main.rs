use std::collections::HashSet;

fn strong_password_checker(password: String) -> i32 {
    let mut password = password;
    let mut steps: u8 = 0;
    steps += (password.len() as u8).checked_sub(20).unwrap_or_default();
    if !password.chars().any(|c| c.is_ascii_digit()) {
        steps += 1;
    }
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        steps += 1;
    }
    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        steps += 1;
    }
    steps += 6u8
        .checked_sub(steps)
        .unwrap_or_default()
        .checked_sub(password.len() as u8)
        .unwrap_or_default();
    println!("{:?}", steps);
    let mut repeated_chars: Vec<char> = Vec::new();
    let mut temp_step: u8 = 0;
    password
        .chars()
        .fold(String::from(&password), |mut mut_pass, c| {
            // println!("{:?}", mut_pass);
            if mut_pass.contains(&[c, c, c].iter().collect::<String>()) {
                // if !repeated_chars.contains(&c) {
                temp_step += 1;
                // repeated_chars.push(c);
                mut_pass = mut_pass.replacen(&[c, c, c].iter().collect::<String>(), "", 1);
                // }
            } else {
                ();
            }
            mut_pass
        });
    // .collect::<Vec<()>>();
    // println!("{},{}", temp_step, steps);
    if temp_step > steps {
        steps = temp_step;
    }
    steps as i32
}

fn main() {
    // let password = "kwei1Aaaiwe";
    let password_1 = "aaa123"; // 1
    let password_2 = "a"; // 5
    let password_3 = "aA1"; // 3
    let password_4 = "1337C0d3"; // 0
    let password_5 = "1111111111"; // 3
    let password_6 = "bbaaaaaaaaaaaaaaacccccc"; // 8
                                                // let password_6 = "bb_aaa_aaa_aaa_aaa_aaa_ccc_ccc";
                                                // let password_6 = "bb_aaA_aa1_aa_baa_paa_cc_0c";
                                                // println!("{}", strong_password_checker(password_1.to_string()));
                                                // println!("{}", strong_password_checker(password_2.to_string()));
                                                // println!("{}", strong_password_checker(password_3.to_string()));
                                                // println!("{}", strong_password_checker(password_4.to_string()));
                                                // println!("{}", strong_password_checker(password_5.to_string()));
    println!("{}", strong_password_checker(password_6.to_string()));
    // println!("{}", remove3duplicates_in_row(password.to_string()));
}
