use std::io;

/// どのコンフィグファイルの操作がなされたか
#[derive(Debug)]
pub enum EditConfig {
    Make,
    Edit(Option<String>, Option<String>),
    Show,
    None,
}

/// codicで発生するErrのバインディング
#[derive(Debug)]
pub enum Error {
    /// io::Error
    CannotOpenConfig {
        error: io::Error
    },
    /// serde_json::Error
    ConfigSyntaxError {
        error: serde_json::Error
    },
    /// reqwest::Error
    CouldNotConnect {
        error: reqwest::Error
    },
    /// リクエストメッセージにエラー
    CodicError(String),
    /// サブコマンドが選択された
    ChosenEditConfig {
        command: EditConfig
    },
}


impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::CannotOpenConfig { error }
    }
}


impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::ConfigSyntaxError {error}
    }
}


impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::CouldNotConnect {error}
    }
}

