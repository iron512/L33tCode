fn main() {
    let s: String = "I".to_string();
    println!("{}", roman_to_int(s));
}

fn roman_to_int(s: String) -> i32 {
    let mut n: i32 = 0;
    let dummy: Vec<char> = s.chars().collect();

    for i in 0..s.len()-1 {
        //println!("{}, {}", c, roman_matching(c));
        let first = roman_matching(dummy[i]);
        let second = roman_matching(dummy[i+1]);

        if first >= second {
            n += first;
        } else {
            n -= first;
        }
    }

    n = n + roman_matching(dummy[dummy.len()-1]);
    return n;
}

fn roman_matching(c: char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => panic!("Invalid Expression")
    }
}
