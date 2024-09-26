use std::{
    env::current_dir,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
    str::FromStr,
};

use tokio::io::{AsyncBufReadExt, BufReader};

use crate::utils::{CpState, TODO_LIST};

use super::{CpId, LsResult, LsResultDir, VIRTUAL_FILE};

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

pub fn rm(dir: &str, target: &str) -> Result<Option<LsResultDir>, String> {
    let target_rm = format!("s3://{}/{}", dir, target);
    let command = Command::new("aws").args(["s3", "rm", &target_rm]).output();
    match command {
        Ok(out) => {
            if out.status.success() {
                return ls_dir(dir);
            } else {
                return Err(format!("rm :{} failed", target_rm));
            }
        }
        Err(e) => {
            return Err(format!("rm failed: {}", e.to_string()));
        }
    }
}

// back a special id
pub async fn cp(from: &str, to: &str, is_upload: bool) -> Result<CpId, Box<dyn std::error::Error>> {
    let special_id = format!("{}-{}", from, to);
    let id = CpId {
        id: special_id.to_string(),
        is_upload,
    };
    // let mut command = Command::new("aws")
    //     .args(["s3", "cp", from, to])
    //     .stdout(Stdio::piped())
    //     .spawn()?;

    let mut command = tokio::process::Command::new("aws")
        .args(["s3", "cp", from, to])
        .stdout(Stdio::piped())
        .spawn()?;

    // match command {
    //     Ok(out) => {
    //         if out.status.success() {
    //             let stdout = String::from_utf8_lossy(out.stdout.as_slice());
    //             dbg!(stdout);

    //             let mut list = TODO_LIST.lock().unwrap();
    //             list.insert(id.clone(), CpState::InProgress(None));
    //             return Ok(id);
    //         } else {
    //             return Err(format!("cp failed! From: {} to {}", from, to));
    //         }
    //     }
    //     Err(e) => {
    //         return Err(format!("cp failed: {}", e.to_string()));
    //     }
    // }
    if let Some(stdout) = command.stdout.take() {
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();
        while let Some(line) = lines.next_line().await? {
            let state = CpState::parse_progress(line.trim());
            // CpState::InProgress(state)
        }
    }

    Ok(id)
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

            target.next().map(|x| {
                ls_res.date.push_str(x);
            });
            target.next().map(|x| {
                ls_res.date.push(' ');
                ls_res.date.push_str(x);
            });

            target.next().map(|target| {
                ls_res.size.replace(usize::from_str_radix(target, 10).unwrap_or(0));
            });
            ls_res.dir = target.next().unwrap().to_string();
            // ls_res.date = format!("{} {}", date, time);
            res.files.push(ls_res);
        }
    }
    res
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
    read_conf().map(|conf| PathBuf::from_str(&conf).unwrap()).ok()
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
    if conf_path.exists() {
        File::open(conf_path.as_path())
    } else {
        File::create_new(conf_path.as_path())
    }
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
