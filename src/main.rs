use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if let Some(val) = args.get_mut(1) {
        let res = beautify(val);
        println!("{}", res);
        return;
    };
    
    println!("No argument provided. Exiting...")
}

fn beautify(val: &mut String) -> String {
    let chars: Vec<char> = val.to_ascii_lowercase().chars().collect();
    let mut new_chars: Vec<char> = vec![];
    let mut is_capital: bool = true;

    for i in 0..chars.len() {
        if chars[i] != ' ' {
            if is_capital {
                new_chars.push(chars[i].to_ascii_uppercase());
                is_capital = false;
                continue;
            }
            new_chars.push(chars[i]);
            is_capital = true;
            continue;
        }
        new_chars.push(chars[i]);
    }
    new_chars.into_iter().collect()
}
