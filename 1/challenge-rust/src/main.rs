//use std::*;
use regex::Regex;
use regex::Match;
use regex::Captures;
use std::env;
use std::fs::read_to_string;

trait MyRegex {
    fn reverse_search<'a>(&self, s: &'a str) -> Option<Match<'a>>;
    fn reverse_captures<'a>(&self, s: &'a str) -> Option<Captures<'a>>;
}

impl MyRegex for Regex {
    fn reverse_search<'a>(&self, s: &'a str) -> Option<Match<'a>> {
        let S = String::from(s);
        for i in (0..S.len()).rev() {
            let m = self.find_at(&s, i);
            match m {
                Some(m) => {
                    return Some(m)
                },
                None => {}
            }
        }
        None
    }

    fn reverse_captures<'a>(&self, s: &'a str) -> Option<Captures<'a>> {
        let S = String::from(s);
        for i in (0..S.len()).rev() {
            let m = self.captures_at(&s, i);
            match m {
                Some(m) => {
                    return Some(m)
                },
                None => {}
            }
        }
        None
    }
}

fn get_matched_digit(caps: &Captures) -> String {
    let d = caps.name("digit");
    match d {
        Some(m) => {
            return m.as_str().to_owned();
        },
        None => {
            let d = caps.name("word");
            match d {
                Some(m) => {
                    let tr_int = match m.as_str() {
                        "one" => 1,
                        "two" => 2,
                        "three" => 3,
                        "four" => 4,
                        "five" => 5,
                        "six" => 6,
                        "seven" => 7,
                        "eight" => 8,
                        "nine" => 9,
                        _ => {panic!()}
                    };
                    return tr_int.to_string();
                },
                None => {}
            }
        }

    };
    "9".to_string()
}

fn print_return(s: &str) -> &str {
    println!("{}", &s);
    &s
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{}", args[1]);
    let file_name = &args[1];
    let re: Regex = Regex::new(r"(?<digit>[0-9])|(?<word>one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let reader: Vec<_> = read_to_string(file_name)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        //.map(String::from)  // make each slice into a string\
        .map(|s| {let last_match = &re.reverse_captures(&s).unwrap();
                    //return &String::from(ss).to_owned()
                    let last_match = get_matched_digit(&last_match);
                    let first_match = re.captures(&s).unwrap();
                    let first_match = get_matched_digit(&first_match);
                    first_match.to_owned() + &last_match
                    })
        //.map(print_return)
        .map(|s| {
            let i: i32 = s.to_string().parse().expect("Not a valid numba");
            i
        })
        .collect();
    for line in &reader {
        println!("{}", line);
    };
    println!("{}, reader.sum", reader.iter().sum::<i32>())
}
