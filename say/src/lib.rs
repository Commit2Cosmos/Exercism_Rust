use std::process::Command;

pub fn encode(n: u64) -> String {
    if n == 0 { return "zero".to_string(); }

    let n = n as usize;
    //* 0 -> 99
    let zero_to_nineteen = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
    let tens = ["", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
    let thousands = ["", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion"];

    let mut res = String::default();
    
    for i in (0..thousands.len()).rev() {
        let th = (n as u128 % 1000_u128.pow((i+1) as u32)) as usize/1000_usize.pow((i) as u32);
        if th > 0 {
            res.push_str(&format!("{} {} ", find_exact(th, &zero_to_nineteen, &tens), thousands[i]));
        }
    }
    
    fn find_exact(mut n: usize, zero_to_nineteen: &[&str], tens: &[&str]) -> String {

        let mut res = String::default();

        if n/100 > 0 {
            res.push_str(zero_to_nineteen[n/100]);
            res.push_str(" hundred ");
        }

        n %= 100;

        if n < 20 {
            res.push_str(zero_to_nineteen[n]);
        } else if n%10 == 0 {
            res.push_str(tens[n/10]);
        } else {
            res.push_str(&format!("{}-{}", tens[n/10], zero_to_nineteen[n%10]));
        }

        res
    }

    res.trim().to_string()
}

pub fn speak(text: String) {
    Command::new("say")
        .arg(text)
        .output()
        .expect("Failed to execute `say` command");
}