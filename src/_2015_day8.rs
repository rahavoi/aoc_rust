use regex::Regex;

pub fn solve(input : &str){
    let mut raw_sum = 0;
    let mut sum = 0;

    input.lines().for_each(|l| {
        raw_sum = raw_sum + l.len();

        let re1 = Regex::new(r"\\x[a-f0-9]{2}").unwrap();
        let re2 = Regex::new(r"\\\\").unwrap();
        let re3 = Regex::new(r#"\\""#).unwrap();

        let mut without_quotes = l.chars();
        without_quotes.next();
        without_quotes.next_back();

        let mut result = re1.replace_all(without_quotes.as_str(), "?");
        let mut result2 = re2.replace_all(&result, "\\");
        let mut result3 = re3.replace_all(&result2, "\"");

        sum = sum + result3.len();

        println!("{l} => {result3} {} : {}", l.len(), result3.len());
    });
    println!("{} - {} = {}", raw_sum, sum, raw_sum - sum);
}