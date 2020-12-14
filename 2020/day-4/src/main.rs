extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::io::{stdin, BufRead};
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let buf = stdin();
    let lines = buf.lock().lines().filter_map(|line_result| line_result.ok());

    let mut passports: Vec<HashMap<String, String>> = Vec::new();
    let mut buf_pport: HashMap<String, String> = HashMap::new();

    for ln in lines {
        if ln == "" {
            passports.push(buf_pport);
            buf_pport = HashMap::new();
        }
        else {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"([a-zA-Z]{3}):([a-zA-Z0-9#]*)").unwrap();
            }
            for cap in RE.captures_iter(&ln) {
                buf_pport.insert(cap[1].to_string(), cap[2].to_string());
            }
        }
    }

    passports.push(buf_pport);

    let mut valid_count = 0;
    let mut valid_count_pt2 = 0;
    for pp in passports {
        if is_valid_passport(&pp){
            valid_count += 1;
        }
        if is_valid_passport_part_two(&pp) {
            valid_count_pt2 += 1;
        }
       }

    println!("valid passports: {}", valid_count);
    println!("valid passports (part 2): {}", valid_count_pt2);
}

fn is_valid_passport(passport: &HashMap<String, String>) -> bool {
    if !passport.contains_key("byr"){
        return false;
    }
    if !passport.contains_key("iyr"){
        return false;
    }
    if !passport.contains_key("eyr"){
        return false;
    }
    if !passport.contains_key("hgt"){
        return false;
    }
    if !passport.contains_key("hcl"){
        return false;
    }
    if !passport.contains_key("ecl"){
        return false;
    }
    if !passport.contains_key("pid"){
        return false;
    }
    // if !passport.contains_key("cid"){
    //     return false;
    // }

    true
}

fn is_valid_passport_part_two(passport: &HashMap<String, String>) -> bool {
    if !contains_valid_date(&passport, "byr", 1920, 2002) {
        println!("invalid byr");
        return false;
    }
    if !contains_valid_date(&passport, "iyr", 2010, 2020){
        println!("invalid iyr");

        return false;
    }
    if !contains_valid_date(&passport, "eyr", 2020, 2030){
        println!("invalid eyr");

        return false;
    }
    if !passport.contains_key("hgt") || !is_valid_height(&passport["hgt"]){
        println!("invalid hgt");

        return false;
    }
    if !passport.contains_key("hcl") || !is_valid_hair_colour(&passport["hcl"]){
        println!("invalid hcl");

        return false;
    }
    if !passport.contains_key("ecl") || !is_valid_eye_colour(&passport["ecl"]){
        println!("invalid ecl");

        return false;
    }
    if !passport.contains_key("pid") || !is_valid_pid(&passport["pid"]){
        println!("invalid pid");

        return false;
    }
    // if !passport.contains_key("cid"){
    //     return false;
    // }

    true
}

fn contains_valid_date(passport: &HashMap<String, String>, key: &str, lower: i32, upper: i32) -> bool {
    if passport.contains_key(key) {
        if let Ok(val) = passport[key].parse::<i32>(){
            if lower <= val && val <= upper {
                return true;
            }
        }
    }
    false
}

fn is_valid_height(height: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d*)(cm|in)").unwrap();
    }

    if let Some(valid_cap) = RE.captures(height){
        let height = valid_cap[1].parse::<i32>().unwrap();
        let unit = &valid_cap[2];
        if unit == "cm" {
            return 150 <= height && height <= 193;
        }
        if unit == "in" {
            return 59 <= height && height <= 76
        }
    }

    false
}

fn is_valid_hair_colour(hair_colour: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
    }

    RE.is_match(hair_colour)
}

fn is_valid_eye_colour(eye_colour: &str) -> bool {
    lazy_static!{
        static ref RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    }

    RE.is_match(eye_colour)
}

fn is_valid_pid(pid: &str) -> bool {
    lazy_static!{
        static ref RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }   

    RE.is_match(pid)
}