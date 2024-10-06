use std::{
    env::current_dir,
    fmt::Display,
    fs::{read_to_string, File, OpenOptions},
    io::Write,
    path::Path,
    process::Command,
    sync::mpsc::channel,
    thread,
};

use super::{ls_dir, LsResult, APP_STATE};

#[derive(Debug, Clone)]
pub struct State {
    /// check aws toolkit terminal install or not
    pub check: bool,
    pub login: bool,
    pub region: String,
    pub output: String,
    pub accsee_key: String,
    pub secret_key: String,
    pub msg: String,
    pub bucket: String,
    pub virtual_dirs: Vec<String>,
    pub current: Option<Vec<LsResult>>,
    pub s3_path: Vec<String>,
    pub notify_page: Option<Pages>,
    pub shares: Option<Vec<ShareItem>>,
}

#[derive(Debug, Clone)]
pub struct ShareItem {
    pub url: String,
    pub name: String,
    pub date: (usize, u8, u8),
    pub during: f32,
}

impl ShareItem {
    pub fn gen_url_success(&self) -> String {
        let date = format!("{}-{}-{}", self.date.0, self.date.1, self.date.2);
        format!(
            "{} generate download url success.\nDuring: {}s\nStart Date: {}",
            self.url, self.during, date
        )
    }
}

impl Display for ShareItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}|{}|{}-{}-{}|{}",
            self.url, self.name, self.date.0, self.date.1, self.date.2, self.during
        ))
    }
}

impl From<&str> for ShareItem {
    fn from(value: &str) -> Self {
        let mut s = value.trim().split("|");
        let url = s.next().unwrap().to_string();
        let name = s.next().unwrap().to_string();
        let date = s.next().unwrap().split("-").collect::<Vec<&str>>();
        let during = s.next().unwrap().to_string().parse().unwrap();
        ShareItem {
            url,
            name,
            date: (
                date[0].parse::<usize>().unwrap(),
                date[1].parse().unwrap(),
                date[2].parse().unwrap(),
            ),
            during,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Pages {
    Upload,
    Sigin,
    Start,
    Bucket,
    Setting,
}

impl Default for State {
    fn default() -> Self {
        Self {
            check: false,
            login: false,
            region: "".to_string(),
            output: "".to_string(),
            accsee_key: "".to_string(),
            secret_key: "".to_string(),
            msg: "".to_string(),
            bucket: "s3".to_string(),
            virtual_dirs: Default::default(),
            current: None,
            s3_path: vec![],
            notify_page: None,
            shares: None,
        }
    }
}

impl State {
    pub fn push_share(&mut self, item: ShareItem) {
        if let Some(shares) = self.shares.as_mut() {
            shares.push(item);
        } else {
            self.shares.replace(vec![item]);
        }
    }
    pub fn sync_shares(init: bool) {
        if init {
            let (sender, receiver) = channel();
            thread::spawn(move || {
                let path = current_dir().unwrap().join("share.cache");
                if !path.exists() {
                    let _ = File::create_new(path.as_path());
                }
                let content = read_to_string(path.as_path()).unwrap();
                if !content.is_empty() {
                    let res = content
                        .split("\n")
                        .map(|s| ShareItem::from(s))
                        .collect::<Vec<ShareItem>>();
                    let _ = sender.send(res);
                }
            });

            thread::spawn(move || {
                let mut state = APP_STATE.lock().unwrap();
                if let Ok(res) = receiver.recv() {
                    state.shares.replace(res);
                }
            });
        } else {
            // new a new thread to set into file
            thread::spawn(move || {
                let path = current_dir().unwrap().join("share.cache");
                // let mut f = if !path.exists() {
                //     File::create_new(path.as_path()).unwrap()
                // } else {
                //     File::open(path.as_path()).unwrap()
                // };
                let mut f = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path.as_path())
                .unwrap();

                let content = {
                    let state = APP_STATE.lock().unwrap();
                    state.shares.as_ref().map(|shares| {
                        shares
                            .iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>()
                            .join("\n")
                    })
                };
                content.map(|content| {
                    let _ = f.write(content.as_bytes());
                });
            });
        }
    }
    pub fn check_toolkit(&mut self) -> bool {
        let command = Command::new("aws").arg("--version").output();
        match command {
            Ok(msg) => {
                self.check = true;
                self.msg = format!(
                    "Success: \n{:?}",
                    String::from_utf8_lossy(msg.stdout.as_slice())
                );
                return true;
                // the msg is json, so use serde_json to parse it
                // dbg!(&msg);
                // let msg_str: Value = serde_json::from_str(&msg).unwrap();
                // // the msg_str is a json object, so get key and value to format it
                // let keys = msg_str.as_object().unwrap().keys();
                // let values = msg_str.as_object().unwrap().values();
                // for (key, value) in keys.zip(values) {
                //     self.msg.push_str( format!("{}: {}\n", key, value).as_str());
                // }
            }
            Err(e) => {
                self.check = false;
                self.msg = format!("Error: \n{:?}\n{}", e, "Please install aws cli toolkit first! Download from https://aws.amazon.com/cli/");
                return false;
            }
        }
    }
    pub fn check_config_credentials(&mut self) {
        // find aws config and credentials file
        // here we should special os system to find the root file (use dirs_next crate)
        if let Some(home) = dirs_next::home_dir() {
            self.msg.push_str("Success: Found home directory!\n");
            let config_path = home.join(".aws").join("config");
            let credentials_path = home.join(".aws").join("credentials");
            self.read_config_credentials(config_path, credentials_path);
        } else {
            self.msg.push_str("Error: Not found home directory!");
        }
    }
    pub fn get_confih_credentials(&mut self) {
        let mut cmd_call = |target: &str| -> Option<String> {
            match Command::new("aws")
                .args(["configure", "get", target])
                .output()
            {
                Ok(out) => {
                    if out.status.success() {
                        return Some(String::from_utf8_lossy(out.stdout.as_slice()).to_string());
                    } else {
                        self.msg = String::from_utf8_lossy(&out.stderr.as_slice()).to_string();
                    }
                }
                Err(e) => self.msg = e.to_string(),
            }
            return None;
        };
        let mut flag = true;
        cmd_call("region").map_or_else(
            || flag = false,
            |res| {
                self.region = res.trim().to_string();
            },
        );
        cmd_call("output").map_or_else(
            || flag = false,
            |res| {
                self.output = res.trim().to_string();
            },
        );
        cmd_call("aws_access_key_id").map_or_else(
            || flag = false,
            |res| {
                self.accsee_key = res;
            },
        );
        cmd_call("aws_secret_access_key").map_or_else(
            || flag = false,
            |res| {
                self.secret_key = res;
            },
        );
        if flag {
            self.check = true;
            self.login = true;
        }
    }
    pub fn read_config_credentials<P>(&mut self, config_path: P, credentials_path: P)
    where
        P: AsRef<Path>,
    {
        // let config_path = PathBuf::from_str("~/.aws/config").unwrap();
        // let credentials_path = PathBuf::from_str("~/.aws/credentials").unwrap();

        if config_path.as_ref().exists() && credentials_path.as_ref().exists() {
            self.msg
                .push_str(format!("Success: Found aws config and credentials file!").as_str());
            // now read the config and credentials file
            let config_content = std::fs::read_to_string(config_path.as_ref()).unwrap();
            let credentials_content = std::fs::read_to_string(credentials_path.as_ref()).unwrap();
            // split `[default]` and get [1]
            let config_lines: Vec<&str> = config_content.split("[default]").collect();
            let credentials_lines: Vec<&str> = credentials_content.split("[default]").collect();
            // split `\r\n`
            let config_lines: Vec<&str> = config_lines[1].trim().split("\r\n").collect();
            let credentials_lines: Vec<&str> = credentials_lines[1].trim().split("\r\n").collect();

            let _ = config_lines.iter().for_each(|x| {
                let kv: Vec<&str> = x.trim().split("=").collect();
                match kv[0].trim() {
                    "region" => self.region = kv[1].trim().to_string(),
                    "output" => self.output = kv[1].trim().to_string(),
                    _ => (),
                }
            });

            let _ = credentials_lines.iter().for_each(|x| {
                let kv: Vec<&str> = x.trim().split("=").collect();
                match kv[0].trim() {
                    "aws_access_key_id" => self.accsee_key = kv[1].trim().to_string(),
                    "aws_secret_access_key" => self.secret_key = kv[1].trim().to_string(),
                    _ => (),
                }
            });
            self.login = true;
        } else {
            self.login = false;
            self.msg
                .push_str(format!("Error: Not found aws config and credentials file!").as_str());
        }
    }

    pub fn sso_login(&mut self) -> bool {
        let command = Command::new("aws")
            .args(["sso", "login", "--profile", "my-dev-profile"])
            .output();
        self.msg.clear();
        match command {
            Ok(res) => {
                if res.status.success() {
                    self.msg = String::from_utf8_lossy(&res.stdout.as_slice()).to_string();
                    return true;
                } else {
                    self.msg = String::from_utf8_lossy(&res.stderr.as_slice()).to_string();
                    return false;
                }
            }
            Err(e) => {
                self.msg = e.to_string();
                return false;
            }
        }
    }
    pub fn set_config_all(&mut self) -> bool {
        let mut cmd_call = |target: &str, replace: &str| -> bool {
            match Command::new("aws")
                .args(["configure", "set", target, replace])
                .output()
            {
                Ok(out) => {
                    if out.status.success() {
                        return true;
                    } else {
                        self.msg = String::from_utf8_lossy(&out.stderr.as_slice()).to_string();
                    }
                }
                Err(e) => self.msg = e.to_string(),
            }
            return false;
        };

        let mut flag = true;
        if !cmd_call("region", &self.region) {
            flag = false;
        }
        if !cmd_call("output", &self.output) {
            flag = false;
        }
        if !cmd_call("aws_access_key_id", &self.accsee_key) {
            flag = false;
        }
        if !cmd_call("aws_secret_access_key", &self.secret_key) {
            flag = false;
        }

        flag
    }
    pub fn ls(&mut self) -> () {
        self.msg.clear();
        match ls_dir(&self.s3_path.join("/")) {
            Ok(res) => {
                res.map(|res| {
                    self.current.replace(res);
                });
            }
            Err(e) => {
                self.msg = e;
            }
        }
    }
}
