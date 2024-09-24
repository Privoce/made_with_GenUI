use std::process::Command;

use super::{LsResult, LsResultDir};

pub fn ls() -> Result<Vec<LsResult>, String> {
    let command = Command::new("aws").arg("s3").arg("ls").output();

    match command {
        Ok(out) => {
            if out.status.success() {
                let msg_str = String::from_utf8_lossy(&out.stdout.as_slice());
                return Ok(format_str_bucket(&msg_str));
            } else {
                return Err(String::from_utf8_lossy(&out.stderr.as_slice()).to_string());
            }
        }
        Err(e) => {
            return Err(format!("Can not list the bucket: {}", e.to_string()));
        }
    }
}

pub fn ls_dir(target: &str) -> Result<Option<LsResultDir>, String> {
    let dir = format!("s3://{}/", target);
    let command = Command::new("aws").args(["s3", "ls", &dir]).output();
    match command {
        Ok(out) => {
            if out.status.success() {
                let msg_str = String::from_utf8_lossy(&out.stdout.as_slice());
                if msg_str.is_empty() {
                    return Ok(None);
                }
                return Ok(Some(format_str_dir(&msg_str)));
            } else {
                return Err(String::from_utf8_lossy(&out.stderr.as_slice()).to_string());
            }
        }
        Err(e) => {
            return Err(format!("Can not list the bucket: {}", e.to_string()));
        }
    }
}

fn format_str_bucket(s: &str) -> Vec<LsResult> {
    let lines: Vec<&str> = s.trim().split("\r\n").collect();
    let mut results = Vec::new();
    for line in lines {
        let mut res = LsResult::default();
        let mut line = line.split_whitespace();
        let date = line.next().unwrap();
        let time = line.next().unwrap();
        let dir = line.next().unwrap();
        res.date = format!("{} {}", date, time);
        res.dir = dir.to_string();
        results.push(res);
    }
    results
}

fn format_str_dir(s: &str) -> LsResultDir {
    let mut res = LsResultDir::default();
    let lines: Vec<&str> = s.trim().split("\r\n").collect();

    for line in lines {
        if line.starts_with("PRE") {
            // split white space and get the last one
            line.split_whitespace()
                .collect::<Vec<&str>>()
                .last()
                .map(|pre| {
                    res.pre.push(pre.trim().replace("/", "").to_string());
                });
        } else {
            let mut ls_res = LsResult::default();
            let mut target = line.split_whitespace();
            
            let date = target.next().unwrap();
            let time = target.next().unwrap();
            target.next().map(|target|{
                ls_res.size = usize::from_str_radix(target, 10).unwrap_or(0);
            });
            ls_res.dir = target.next().unwrap().to_string();
            ls_res.date = format!("{} {}", date, time);
            res.files.push(ls_res);
        }
    }
    res
}

#[cfg(test)]
mod t {
    #[test]
    fn test_ls() {
        let results = super::ls();
        dbg!(results.unwrap());
        // assert!(results.is_ok());
    }
    #[test]
    fn test_ls_dir() {
        let results = super::ls_dir("yarddesign");
        dbg!(results.unwrap());
    }
}
