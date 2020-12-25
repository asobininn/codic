use serde::{Serialize, Deserialize};
use std::io::{Read, BufReader, Write, BufWriter};
use std::fs::File;
use std::path::PathBuf;
use super::Error;

/// codicのコンフィグファイルのデータ
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    token: String,
    casing: String,
}


impl Config {
    /// コンフィグファイルの初期生成時のcasing
    pub const DEFAULT_CASING: &'static str = "camel";


    /// 空のConfigを生成
    pub fn new(token: &str, casing: &str) -> Self {
        Config {
            token: token.to_string(),
            casing: casing.to_string(),
        }       
    }


    /// コンフィグファイルから生成
    pub fn from_file(path: &PathBuf) -> Result<Self, Error> {
        let mut f = BufReader::new(File::open(path)?);
        let mut json = String::new();
        f.read_to_string(&mut json)?;
        let config: Config = serde_json::from_str(&json)?;
        Ok(config)
    }


    /// Configの持つデータをコンフィグファイルに書き込む
    pub async fn write_file(&self, path: &PathBuf) -> Result<(), Error>{
        let json = serde_json::to_string(&self)?;
        let mut f = BufWriter::new(File::create(path)?);
        f.write(json.as_bytes())?;
        Ok(())
    }


    pub fn token(&self) -> &str {
        &self.token
    }


    pub fn token_mut(&mut self) -> &mut String {
        &mut self.token
    }


    pub fn casing(&self) -> &str {
        &self.casing
    }


    pub fn casing_mut(&mut self) -> &mut String {
        &mut self.casing
    }


    pub async fn make_default_file(path: &PathBuf) -> Result<(), Error> {
        Config::new("", Config::DEFAULT_CASING)
            .write_file(path).await?;
        Ok(())
    }


    /// コンフィグファイルを編集する
    /// Option値がNoneの場合は、その箇所を変更しない
    pub async fn edit(path: &PathBuf, token: Option<&str>, casing: Option<&str>) -> Result<(), Error> {
        let mut config = Config::from_file(path)?;
        if let Some(token) = token {
            *config.token_mut() = token.to_string();
        }
        if let Some(casing) = casing {
            *config.casing_mut() = casing.to_string();
        }
        config.write_file(path).await?;
        Ok(())
    }

}
