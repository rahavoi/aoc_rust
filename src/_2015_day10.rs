use std::fmt::format;

pub fn solve(input : &str){
    let mut data = input.to_string();

    for i in 0..40 {
        println!("{i}");
        let mut copy = data.as_bytes();
        let mut cur_idx = 1;
        let mut out = "".to_owned();
        let mut first_byte = &copy[0];
        let mut cur_char = *first_byte as char;
        let mut  cur_count = 1;

        while cur_idx < copy.len() {
            let u8 = &copy[cur_idx];
            let next = *u8 as char;

            if cur_char == next {
                cur_count = cur_count + 1;
            } else {
                out = [out, cur_count.to_string(), cur_char.to_string()].concat();
                cur_count = 1;
                cur_char = next;
            }

            cur_idx = cur_idx + 1;
        }

        out = [out, cur_count.to_string(), cur_char.to_string()].concat();

        data = out;
        //println!("{}", data.len());
    }

    println!("{}", data.len());
}