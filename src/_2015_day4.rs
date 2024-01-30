pub fn solve(key : &str) {
    let mut num = 0;
    let mut digest = "".to_string();

    while !digest.starts_with("000000") {
        num = num + 1;
        let mut copy = key.to_string();
        copy.push_str(&num.to_string());

        digest = format!("{:x}", md5::compute(copy));
    }

    println!("{}", num);
}