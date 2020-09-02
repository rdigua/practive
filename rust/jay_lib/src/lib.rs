use std::path::Path;

use regex::Regex;
//use reqwest::ClientBuilder;
use reqwest::Client;
//use reqwest::header::ETAG;

pub fn exist_dir(p: &str) -> bool {
    Path::new(p).exists() && Path::new(p).is_dir()
}

pub fn exist_file(p: &str) -> bool {
    Path::new(p).exists() && Path::new(p).is_file()
}

fn strtou32(s: &str) -> u32 {
    let stu = s.to_string();
    let picks: String = stu.chars().filter(|x| x.is_digit(10)).collect();

    if picks.len() == 0 {
        return 0;
    }

    return picks.parse::<u32>().unwrap();
}

pub fn getcon(link: &str) -> String {
    let c=reqwest::Client::new().get(link).send().unwrap().text().unwrap();
    c.to_string()

//    let client = Client::new();
//    let  r = client.get(link).text()?;
//    if r.status().is_success{
//        return r.text();
//    } else {
//        return String::from("?");
//    }
    /*
    match reqwest::get(link){


    }
    
    ?.text()?;
    r
    
    let mut body = String::new();
    let client = reqwest::Client::new();
    let response = client.head(link).send()?;
    if response.status().is_success(){
        response.to_string(&body);
    }
    body
    */
}

pub fn getlistvec(re: &str, content: &str) -> Vec<String> {
    let re1 = Regex::new(re).unwrap();
    //content);
    let mut rveg: Vec<String> = Vec::new();
//re.captures_iter
    for cap in re1.captures_iter(&content) {
        rveg.push(cap[1].to_string());
    }
    rveg
}

pub fn getliststr(re: &str, content: &str) -> String {
    let re1 = Regex::new(re).unwrap();
    let mut rstr = String::new();
    for cap in re1.captures_iter(&content) {
        rstr.push_str(&cap[1].to_string());
        rstr.push_str(";");
        
    }
    rstr
}
