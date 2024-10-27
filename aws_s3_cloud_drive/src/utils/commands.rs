use std::{
    env::current_dir,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
    process::Command,
    str::FromStr,
};

use crate::utils::CpState;

use super::{CpId, LsResult, Req, APP_STATE, LOAD_LIST, VIRTUAL_FILE};

pub fn ls_dir(target: &str) -> Result<Option<Vec<LsResult>>, String> {
    let mut is_bucket = false;
    let dir = if target.is_empty() {
        is_bucket = true;
        "".to_string()
    } else {
        format!("s3://{}/", target)
    };
    let command = Command::new("aws").args(["s3", "ls", &dir]).output();
    match command {
        Ok(out) => {
            if out.status.success() {
                let msg_str = String::from_utf8_lossy(&out.stdout.as_slice());
                if msg_str.is_empty() {
                    return Ok(None);
                }
                return Ok(Some(format_str_dir(&msg_str, is_bucket)));
            } else {
                return Err(String::from_utf8_lossy(&out.stderr.as_slice()).to_string());
            }
        }
        Err(e) => {
            return Err(format!("Can not list the bucket: {}", e.to_string()));
        }
    }
}

pub async fn rm(target: &str) -> Result<(), String> {
    // let target_rm = format!("s3://{}/{}", dir, target);
    let command = Command::new("aws").args(["s3", "rm", &target]).output();
    match command {
        Ok(out) => {
            if out.status.success() {
                let mut state = APP_STATE.lock().unwrap();
                state.req = Req::Rm;
                return Ok(());
            } else {
                return Err(format!("rm :{} failed", target));
            }
        }
        Err(e) => {
            return Err(format!("rm failed: {}", e.to_string()));
        }
    }
}

// back a special id
pub async fn cp(from: &str, to: &str, id: &CpId) -> Result<CpId, Box<dyn std::error::Error>> {
    let command = Command::new("aws").args(["s3", "cp", from, to]).output();

    match command {
        Ok(out) => {
            if out.status.success() {
                // 仅在更新状态时获取锁，减小锁的持有时间
                let mut list = LOAD_LIST.lock().unwrap();
                list.insert(id.clone(), CpState::Completed);
                Ok(id.clone())
            } else {
                let mut list = LOAD_LIST.lock().unwrap();
                list.insert(id.clone(), CpState::Failed);
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    String::from_utf8_lossy(&out.stderr),
                )))
            }
        }
        Err(_) => {
            let mut list = LOAD_LIST.lock().unwrap();
            list.insert(id.clone(), CpState::Failed);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "AWS S3 cp failed",
            )));
        }
    }
}

pub fn share(target: &str, time: f32) -> Result<String, String> {
    let cmd = Command::new("aws")
        .args([
            "s3",
            "presign",
            target,
            "--expires-in",
            time.to_string().as_str(),
        ])
        .output();
    match cmd {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(e.to_string()),
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
        res.date.replace(format!("{} {}", date, time));
        res.name = dir.to_string();
        results.push(res);
    }
    results
}

fn format_str_dir(s: &str, is_bucket: bool) -> Vec<LsResult> {
    let mut res = Vec::new();
    let lines: Vec<&str> = if s.trim().contains("\r\n") {
        s.trim().split("\r\n").map(|x| x.trim()).collect()
    } else {
        s.trim().split("\n").map(|x| x.trim()).collect()
    };

    if is_bucket {
        format_str_bucket(s)
    } else {
        for line in lines {
            if line.starts_with("PRE") {
                // split white space and get the last one
                line.split_whitespace()
                    .collect::<Vec<&str>>()
                    .last()
                    .map(|pre| {
                        res.push(LsResult::bucket(&pre.trim().replace("/", "")));
                    });
            } else {
                if line.is_empty() {
                    continue;
                }
                let mut ls_res = LsResult::default();
                let mut target = line.split_whitespace();
                let mut date = String::new();
                target.next().map(|x| {
                    date.push_str(x);
                });
                target.next().map(|x| {
                    date.push_str(" ");
                    date.push_str(x);
                    ls_res.date.replace(date);
                });

                target.next().map(|target| {
                    ls_res
                        .size
                        .replace(usize::from_str_radix(target, 10).unwrap_or(0));
                });
                target.next().map(|target| {
                    ls_res.name = target.to_string();
                });

                // ls_res.date = format!("{} {}", date, time);
                res.push(ls_res);
            }
        }
        res
    }
}

pub fn set_conf(k: &str) -> Result<(), std::io::Error> {
    let mut f = read_or_create("conf.conf")?;
    // set back
    f.write(format!("{}\n", k).as_bytes()).map(|_| ())
}

pub fn read_conf() -> Result<String, std::io::Error> {
    let mut f = read_or_create("conf.conf")?;
    let mut buf = String::new();
    let _ = f.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn conf_static() -> Option<PathBuf> {
    read_conf()
        .map(|conf| PathBuf::from_str(&conf).unwrap())
        .ok()
}

pub fn push_virtual(item: &str) -> Result<(), std::io::Error> {
    let mut f = read_or_create(VIRTUAL_FILE)?;
    let mut buf = String::new();
    let _ = f.read_to_string(&mut buf)?;
    // add to file
    if !buf.is_empty() {
        buf.push('\n');
    }
    buf.push_str(item);
    // set back
    f.write(buf.as_bytes()).map(|_| ())
}

pub fn read_virtual() -> Result<Vec<String>, std::io::Error> {
    let mut f = read_or_create(VIRTUAL_FILE)?;
    let mut buf = String::new();
    let _ = f.read_to_string(&mut buf)?;
    Ok(buf
        .split('\n')
        .map(|x| x.to_string())
        .collect::<Vec<String>>())
}

pub fn remove_virtual(target: &str) -> Result<Vec<String>, std::io::Error> {
    let mut f = read_or_create(VIRTUAL_FILE)?;
    let mut buf = String::new();
    let _ = f.read_to_string(&mut buf)?;
    let mut list = buf
        .split('\n')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    list.retain(|x| x != target);
    Ok(list)
}

pub fn read_or_create(target: &str) -> std::io::Result<File> {
    let current_path = current_dir().unwrap();
    let conf_path = current_path.join(target);
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(conf_path.as_path())
}

pub fn format_s3_path(path: &Vec<String>) -> String {
    format!("s3://{}/", path.join("/"))
}

#[cfg(test)]
mod t {
    #[test]
    fn test_ls() {
        // let results = super::ls();
        // dbg!(results.unwrap());
        // assert!(results.is_ok());
    }
    #[test]
    fn test_ls_dir() {
        let results = super::ls_dir("yarddesign");
        dbg!(results.unwrap());
    }
    // #[test]
    // fn test_cp() {
    //     let results = super::cp(
    //         r#"E:\pratice_imgs\makepad_taobao-main.zip"#,
    //         "s3://yarddesign/pratice_imgs/",
    //         true,
    //     ).await?;
    //     dbg!(results);
    // }
}
