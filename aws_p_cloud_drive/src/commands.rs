use std::process::Command;

use crate::aws_structs::LsResult;

pub fn ls(bucket: &str) -> Result<Vec<LsResult>, String>{
    let command = Command::new("aws").arg(bucket).arg("ls").output();

    match command {
        Ok(msg) => {
            let msg_str = String::from_utf8_lossy(msg.stdout.as_slice());
            return Ok(format_str(&msg_str));
        }
        Err(e) => {
            return Err(
                format!("Can not list the bucket: {}", e.to_string())
            );
        }
    }
}

fn format_str(s: &str) -> Vec<LsResult>{
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


#[cfg(test)]
mod t{
    #[test]
    fn test_ls(){
        let results = super::ls("s3");
        dbg!(results.unwrap());
        // assert!(results.is_ok());
    }
}

