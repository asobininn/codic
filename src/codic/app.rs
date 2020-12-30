use std::path::PathBuf;
use super::{Config, Error, error::EditConfig, clap_params::*};

/// codicアプリケーション本体
pub struct App {
    text: String,
    config: Config,
}


impl App {

    /// 空のAppを生成
    pub fn new() -> Self {
        App {
            text: String::new(),
            config: Config::new("", "")
        }
    }


    /// 引数の文字列からApp生成
    pub fn from_input(text: &str, token: &str, casing: &str) -> Self {
        App {
            text: text.to_string(),
            config: Config::new(token, casing)
        }
    }


    /// コンフィグファイルからApp生成
    /// textは空
    pub fn from_config_file(path: &PathBuf) -> Result<Self, Error> {
        let config = Config::from_file(path)?;
        Ok(App {
            text: String::new(),
            config
        })
    }


    pub fn text_mut(&mut self) -> &mut String {
        &mut self.text
    }


    /// clap machesを解析し、必要に応じてtext、token、casingを変更する
    pub async fn take_params(mut self) -> Result<Self, Error> {
        let matches = get_matches();
        // subcommand 
        if let Some(sub) = matches.subcommand_matches("config") {
            if sub.is_present("make") {
                return Err(Error::ChosenEditConfig{command: EditConfig::Make});
            } else if sub.is_present("remove") {
                return Err(Error::ChosenEditConfig{command: EditConfig::Remove});
            } else if sub.is_present("show") {
                return Err(Error::ChosenEditConfig{command: EditConfig::Show});
            } else if let Some(edit) = sub.subcommand_matches("edit") {
                let mut token: Option<String> = None;
                let mut casing: Option<String> = None;
                if let Some(o) = edit.value_of("config token") {
                    token = Some(o.to_string());
                }
                if edit.is_present("config casing") {
                    //casing = Some("upper underscore".to_string());
                    for (i, c) in CONFIG_CASINGS.iter().enumerate() {
                        if edit.is_present(c) {
                            casing = Some(CASINGS[i].to_string());
                        }
                    }
                }
                if token.is_some() || casing.is_some() {
                    return Err(Error::ChosenEditConfig{command: EditConfig::Edit(token, casing)});
                }
            }
            return Err(Error::ChosenEditConfig{command: EditConfig::None});
        }
        // main
        if let Some(o) = matches.value_of("text") {
            self.text = o.to_string();
        }
        // casing
        for c in &CASINGS {
            if matches.is_present(c) {
                *self.config.casing_mut() = c.to_string();
            }
        }
        Ok(self)
    }


    /// アプリケーションを実行する
    /// # return
    /// Ok(String): 翻訳された文字列
    /// Err(codic::Error): サブコマンド選択、または実行時エラーの情報
    pub async fn run(&self) -> Result<String, Error> {
        let url = format!("https://api.codic.jp/v1/engine/translate.json?text={}&casing={}",
                            self.text, self.config.casing());
        let header = format!("Bearer {}", self.config.token());
        let json: serde_json::Value = reqwest::Client::new()
            .get(&url)
            .header(reqwest::header::AUTHORIZATION, &header)
            .send().await?
            .json().await?;
        if let Some(errors) = json.get("errors") {
            return Err(Error::CodicError(
                errors[0]["message"].to_string()
                .trim_matches('\"').to_string()
            ));
        } else if let Some(_) = json[0].get("successful") {
            return Ok(
                json[0]["translated_text"].to_string()
                .trim_matches('\"').to_string()
            );
        }
        Err(Error::CodicError("その他のエラー".to_string()))
    }


}

