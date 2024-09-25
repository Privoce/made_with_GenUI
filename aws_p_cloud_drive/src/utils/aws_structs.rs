#[allow(unused_imports)]
use std::default;

#[derive(Debug, Clone, Default)]
pub struct LsResult{
    pub date: String,
    pub dir: String,
    pub size: usize,
}

#[derive(Debug, Clone, Default)]
pub struct LsResultDir{
    pub pre: Vec<String>,
    pub files: Vec<LsResult>
}

#[derive(Debug, Clone)]
pub enum S3Data{
    Bucket(Vec<LsResult>),
    Dir(Option<LsResultDir>)
}
impl Default for S3Data {
    fn default() -> Self {
        S3Data::Bucket(vec![])
    }
}

#[derive(Debug, Clone, Default)]
pub enum Handles{
    Ls,
    Delete,
    Downlaod,
    #[default]
    None
}

#[derive(Debug, Clone, Default)]
pub struct FileTableItem{
    pub name: String,
    pub size: usize,
}