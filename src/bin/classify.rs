extern crate regex;
use regex::Regex;
use sublime_fuzzy::best_match;

use std::fs::File;
use std::io::{prelude::*, BufReader};

use serde::{Deserialize, Serialize};
// use serde_json::Result;
// use serde_json::json;

use std::error::Error;

fn split_once(in_string: &str) -> (&str, &str) {
    let mut splitter = in_string.splitn(2, ':');
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    (first, second)
}

fn split_twice(in_string: &str) -> (&str, &str, &str) {
    let mut splitter = in_string.splitn(2, ',');
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    let third = splitter.next().unwrap();
    (first, second, third)
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut mystring: Vec<String> = Vec::new();
    let mut myindexes: Vec<String> = Vec::new();
    let file = File::open("./all_reqs.csv")?;
    // let file = File::open("./src/reqs-mini.csv")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let vx: Vec<String> = line?.split(",").map(|x| x.to_string()).collect();
        mystring.push(vx[2].clone());
        myindexes.push(vx[1].clone());
    }
    let mut mycounter = 0;
    let mut all_crses: Vec<CrseLevel> = vec![];
    for input in mystring.clone() {
        mycounter += 1;
        let expr = input.clone();

        let single_req: Vec<String> = match expr.contains(":") {
            true => {
                let (_beg, rest) = split_once(&expr);
                rest.split(";").map(|x| x.to_string()).collect()
            }
            false => expr.split(";").map(|x| x.to_string()).collect(),
        };

        let list_pre_req: Vec<PreReq> = classify(single_req);
        let crse_lvl = CrseLevel {
            index: mycounter,
            crse: myindexes[(mycounter - 1) as usize].clone(),
            input: input.clone(),
            list_pre_reqs: list_pre_req,
        };

        all_crses.push(crse_lvl)
    }
    println!("{}", serde_json::json!(all_crses));
    Ok(())
}

fn extract_crs_nbrs(pat: String) -> Vec<String> {
    let starts: Vec<(usize, usize, String)> = Regex::new(r"[A-Z]{3} [0-9]{3}")
        .unwrap()
        .find_iter(&pat)
        .map(|mat| (mat.start(), mat.end(), mat.as_str().to_string()))
        .collect();

    let mut counter = 0;

    let mut final_results: Vec<String> = vec![];

    for (x, _y, _z) in starts.clone() {
        let end_index = match (counter + 1) < starts.len() {
            true => starts[counter + 1].0 - 1,
            false => pat.len(),
        };

        let mystr = pat.clone();
        let (_, end) = mystr.split_at(x);
        let (target, _) = end.split_at(end_index - x);

        //
        let res = match Regex::new(r"[A-Z]{3} [0-9]{3}.*or [0-9]{3}")
            .unwrap()
            .captures(&target)
        {
            Some(newline) => {
                let data = newline.get(0).map_or("", |m| m.as_str());
                let pattern = Regex::new(r"or ").unwrap();
                pattern.replace_all(&data, "").to_string()
            }
            None => target.to_string(),
        };

        let pattern = Regex::new(r"[^A-Z0-9 ]").unwrap();
        let newline: String = pattern.replace_all(&res, "").to_string().trim().to_string();
        let mut holder: Vec<String> = newline.split(" ").map(|x| x.to_string()).collect();
        let numbers: Vec<String> = holder.drain(1..).collect();
        let subject = &holder.clone()[0];

        let mut results: Vec<String> = vec![];
        for nbr in &numbers {
            let s = format!("{} {}", subject, nbr);
            results.push(s);
        }
        final_results = [&final_results[..], &results[..]].concat();
        counter += 1;
    }

    final_results
}

fn is_min_pre_req(rq: String) -> bool {
    let s = rq;
    let search = "minimum";
    let minscore = match best_match(search, &s) {
        Some(result) => result.score(),
        None => 0,
    };
    if minscore > 100 {
        return true;
    }
    false
}

fn is_max_pre_req(rq: String) -> bool {
    let s = rq;
    let search = "maximum";
    let minscore = match best_match(search, &s) {
        Some(result) => result.score(),
        None => 0,
    };
    if minscore > 100 {
        return true;
    }
    false
}

fn is_grad_pre_req(rq: String) -> bool {
    let s = rq;
    let search = "graduate student";
    let minscore = match best_match(search, &s) {
        Some(result) => result.score(),
        None => 0,
    };
    if minscore > 100 {
        return true;
    }
    false
}

fn is_non_degree_seeking_pre_req(rq: String) -> bool {
    let s = rq;
    let search = "nondegree seeking";
    let minscore = match best_match(search, &s) {
        Some(result) => result.score(),
        None => 0,
    };
    if minscore > 100 {
        return true;
    }
    false
}
fn is_degree_seeking_pre_req(rq: String) -> bool {
    let s = rq;
    let search = "degree seeking";
    let minscore = match best_match(search, &s) {
        Some(result) => result.score(),
        None => 0,
    };
    if minscore > 100 {
        return true;
    }
    false
}
fn is_freshy_pre_req(rq: String) -> bool {
    let s = rq;
    let search = "freshmen";
    let minscore = match best_match(search, &s) {
        Some(result) => result.score(),
        None => 0,
    };
    if minscore > 100 {
        return true;
    }
    false
}
fn is_soph_pre_req(rq: String) -> bool {
    let s = rq;
    let search = "sophomore";
    let minscore = match best_match(search, &s) {
        Some(result) => result.score(),
        None => 0,
    };
    if minscore > 100 {
        return true;
    }
    false
}
fn is_junior_pre_req(rq: String) -> bool {
    let s = rq;
    let search = "junior";
    let minscore = match best_match(search, &s) {
        Some(result) => result.score(),
        None => 0,
    };
    if minscore > 100 {
        return true;
    }
    false
}
fn is_senior_pre_req(rq: String) -> bool {
    let s = rq;
    let search = "senior";
    let minscore = match best_match(search, &s) {
        Some(result) => result.score(),
        None => 0,
    };
    if minscore > 100 {
        return true;
    }
    false
}
fn is_about_credit_pre_req(rq: String) -> bool {
    let s = rq;
    let search = "credit is allowed for only";
    let minscore = match best_match(search, &s) {
        Some(result) => result.score(),
        None => 0,
    };
    if minscore > 100 {
        return true;
    }
    false
}
fn is_barrett_honors_pre_req(rq: String) -> bool {
    let s = rq;
    let search = "barrett honors";
    let minscore = match best_match(search, &s) {
        Some(result) => result.score(),
        None => 0,
    };
    if minscore > 100 {
        return true;
    }
    false
}
fn is_teachers_pre_req(rq: String) -> bool {
    let s = rq;
    let search = "Teachers College";
    let minscore = match best_match(search, &s) {
        Some(result) => result.score(),
        None => 0,
    };
    if minscore > 100 {
        return true;
    }
    false
}
fn is_mary_lou_fulton_pre_req(rq: String) -> bool {
    let s = rq;
    let search = "Mary Lou Fulton Teachers College";
    let minscore = match best_match(search, &s) {
        Some(result) => result.score(),
        None => 0,
    };
    if minscore > 100 {
        return true;
    }
    false
}
fn is_cor_better_pre_req(rq: String) -> bool {
    let s = rq;
    let search = "with C or better";
    let minscore = match best_match(search, &s) {
        Some(result) => result.score(),
        None => 0,
    };
    if minscore > 100 {
        return true;
    }
    false
}
fn is_additional_hours_pre_req(rq: String) -> bool {
    let s = rq;
    let search = "additional hours";
    let minscore = match best_match(search, &s) {
        Some(result) => result.score(),
        None => 0,
    };
    if minscore > 100 {
        return true;
    }
    false
}
fn is_cannot_enroll_pre_req(rq: String) -> bool {
    let s = rq;
    let search = "may not enroll in";
    let minscore = match best_match(search, &s) {
        Some(result) => result.score(),
        None => 0,
    };
    if minscore > 100 {
        return true;
    }
    false
}
fn is_earned_credit_hours_pre_req(rq: String) -> bool {
    let s = rq;
    let search = "earned credit hours";
    let minscore = match best_match(search, &s) {
        Some(result) => result.score(),
        None => 0,
    };
    if minscore > 100 {
        return true;
    }
    false
}
fn is_mathematics_placement_test_pre_req(rq: String) -> bool {
    let s = rq;
    let search = "math placement test";
    let minscore = match best_match(search, &s) {
        Some(result) => result.score(),
        None => 0,
    };
    if minscore > 100 {
        return true;
    }
    false
}
fn is_aleks_pre_req(rq: String) -> bool {
    let s = rq;
    let search = "ALEKS score";
    let minscore = match best_match(search, &s) {
        Some(result) => result.score(),
        None => 0,
    };
    if minscore > 100 {
        return true;
    }
    false
}

#[derive(Debug, Serialize, Deserialize)]
struct PreReqClassified {
    is_min: bool,
    is_max: bool,
    is_grad: bool,
    is_non_degree_seeking: bool,
    is_degree_seeking: bool,
    is_freshy: bool,
    is_about_credit: bool,
    is_barrett_honors: bool,
    is_c_or_better: bool,
    is_additional_hours: bool,
    is_cannot_enroll: bool,
    is_earned_credit_hours: bool,
    is_mary_lou_fulton: bool,
    is_teachers: bool,
    is_soph: bool,
    is_junior: bool,
    is_senior: bool,
    is_mathematics_placement_test: bool,
    is_aleks: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct PreReq {
    original: String,
    classified: PreReqClassified,
    mentioned: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CrseLevel {
    index: i32,
    crse: String,
    input: String,
    list_pre_reqs: Vec<PreReq>,
}

fn classify(single_req: Vec<String>) -> Vec<PreReq> {
    let mut list_pre_req: Vec<PreReq> = vec![];
    for req in single_req {
        let rq = req.trim().to_string();
        let is_min = is_min_pre_req(rq.clone());
        let is_max = is_max_pre_req(rq.clone());
        let is_grad = is_grad_pre_req(rq.clone());
        let is_non_degree_seeking = is_non_degree_seeking_pre_req(rq.clone());
        let is_degree_seeking = is_degree_seeking_pre_req(rq.clone());
        let is_freshy = is_freshy_pre_req(rq.clone());
        let is_about_credit = is_about_credit_pre_req(rq.clone());
        let is_barrett_honors = is_barrett_honors_pre_req(rq.clone());
        let is_c_or_better = is_cor_better_pre_req(rq.clone());
        let is_additional_hours = is_additional_hours_pre_req(rq.clone());
        let is_cannot_enroll = is_cannot_enroll_pre_req(rq.clone());
        let is_earned_credit_hours = is_earned_credit_hours_pre_req(rq.clone());
        let is_mary_lou_fulton = is_mary_lou_fulton_pre_req(rq.clone());
        let is_teachers = is_teachers_pre_req(rq.clone());

        let is_soph = is_soph_pre_req(rq.clone());
        let is_junior = is_junior_pre_req(rq.clone());
        let is_senior = is_senior_pre_req(rq.clone());
        let is_mathematics_placement_test = is_mathematics_placement_test_pre_req(rq.clone());
        let is_aleks = is_aleks_pre_req(rq.clone());

        let obj = PreReqClassified {
            is_min: is_min,
            is_max: is_max,
            is_grad: is_grad,
            is_non_degree_seeking: is_non_degree_seeking,
            is_degree_seeking: is_degree_seeking,
            is_freshy: is_freshy,
            is_about_credit: is_about_credit,
            is_barrett_honors: is_barrett_honors,
            is_c_or_better: is_c_or_better,
            is_additional_hours: is_additional_hours,
            is_cannot_enroll: is_cannot_enroll,
            is_earned_credit_hours: is_earned_credit_hours,
            is_mary_lou_fulton: is_mary_lou_fulton,
            is_teachers: is_teachers,

            is_soph: is_soph,
            is_junior: is_junior,
            is_senior: is_senior,
            is_mathematics_placement_test: is_mathematics_placement_test,
            is_aleks: is_aleks,
        };
        // let pattern = Regex::new(r"(?P<beg>[A-Z]{3} [0-9]{3})").unwrap();
        // let xtel: Vec<String> = pattern
        //     .find_iter(&rq)
        //     .map(|mat| mat.as_str().to_string())
        //     .collect();

        let xtel: Vec<String> = extract_crs_nbrs(rq.to_string());

        let evens: Vec<String> = xtel
            .iter()
            .filter(|x| x.len() == (6 + 1))
            .cloned()
            .collect();

        let prereq = PreReq {
            original: rq,
            classified: obj,
            mentioned: evens,
        };

        list_pre_req.push(prereq);
    }
    list_pre_req
}
