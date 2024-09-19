use std::{path::Path, process::Command};

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
        }
    }
    
}

impl State {
    pub fn check_toolkit(&mut self) -> bool{
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

    pub fn sso_login(&mut self) -> bool{
        let command = Command::new("aws").args(["sso", "login", "--profile", "my-dev-profile"]).output();
        self.msg.clear();
        match command{
            Ok(res) => {
                if res.status.success(){
                    self.msg = String::from_utf8_lossy(&res.stdout.as_slice()).to_string();
                    return true;
                }else{
                    self.msg = String::from_utf8_lossy(&res.stderr.as_slice()).to_string();
                    return false;
                }
            },
            Err(e) => {
                self.msg = e.to_string();
                return false;
            },
        }
    }
}
