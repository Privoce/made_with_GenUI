#[allow(unused_imports)]
use std::default;

use regex::Regex;

#[derive(Debug, Clone, Default)]
pub struct LsResult {
    pub date: Option<String>,
    pub name: String,
    pub size: Option<usize>,
}

impl LsResult {
    pub fn bucket(s: &str) -> Self {
        Self {
            date: None,
            name: s.to_string(),
            size: None,
        }
    }
}

// #[derive(Debug, Clone, Default)]
// pub struct LsResultDir {
//     pub pre: Vec<String>,
//     pub files: Vec<LsResult>,
// }

// impl LsResultDir {
//     pub fn is_empty(&self) -> bool {
//         self.pre.is_empty() && self.files.is_empty()
//     }
//     pub fn unified(&self) -> Vec<LsResult> {
//         let mut res = self.files.clone();
//         res.extend(self.pre.iter().map(|x| LsResult {
//             date: Default::default(),
//             dir: x.to_string(),
//             size: None,
//         }));
//         res
//     }
// }

// #[derive(Debug, Clone)]
// pub enum S3Data {
//     Bucket(Vec<LsResult>),
//     Dir(Option<LsResultDir>),
// }
// impl Default for S3Data {
//     fn default() -> Self {
//         S3Data::Bucket(vec![])
//     }
// }

#[derive(Debug, Clone, Default)]
pub enum Handles {
    Ls,
    Delete,
    Downlaod,
    #[default]
    None,
}

#[derive(Debug, Clone, Default)]
pub struct FileTableItem {
    pub name: String,
    pub size: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Hash, Eq)]
pub struct CpId {
    pub id: String,
    pub is_upload: bool,
}

impl CpId {
    pub fn new(from: &str, to: &str, is_upload: bool) -> Self{
        let special_id = format!("{}-{}", from, to);
        CpId {
            id: special_id.to_string(),
            is_upload,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CpState {
    InProgress(Option<ProgressIn>),
    Completed,
    Failed,
}

impl Default for CpState {
    fn default() -> Self {
        CpState::InProgress(None)
    }
}

#[derive(Debug, Clone)]
pub struct ProgressIn {
    pub total: String,
    pub current: String,
    pub timing: String,
}

impl CpState {
    // Completed 256.0 KiB/15.6 MiB (81.0 KiB/s) with 1 file(s)
    // Completed 1.6 KiB/1.6 KiB (476 Bytes/s) with 1 file(s) remaining\rupload: ..\\..\\..\\..\\..\\pratice_imgs\\upload.svg to s3://yarddesign/pratice_imgs/upload.svg\r\n
    pub fn parse_progress(line: &str) -> CpState {
        // eat Completed
        let re = Regex::new(
            r"Completed\s([0-9.]+ [A-Za-z]+)/([0-9.]+ [A-Za-z]+)\s\(([0-9.]+ [A-Za-z]+/s)\)",
        )
        .unwrap();

        if let Some(captures) = re.captures(line) {
            let current = captures.get(1).unwrap().as_str().to_string();
            let total = captures.get(2).unwrap().as_str().to_string();
            let timing = captures.get(3).unwrap().as_str().to_string();

            if current != total {
                CpState::InProgress(Some(ProgressIn {
                    total,
                    current,
                    timing,
                }))
            } else {
                CpState::Completed
            }
        } else {
            CpState::Failed
        }
    }
}
