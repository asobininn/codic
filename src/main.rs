use codic::codic::{error, App, Config, Error};
use std::path::PathBuf;


/// Result型にまとめるためのバインディング
async fn codic_run(path: &PathBuf) -> Result<String, Error> {
    Ok(App::from_config_file(path)?
        .take_params().await?
        .run().await?
    )
}


#[tokio::main]
async fn main() {
    // コンフィグファイルの絶対パス
    let config_file = ".codic_config.json";
    let mut path = std::env::current_exe().unwrap()
        .parent().unwrap()
        .to_path_buf();
    path.push(&config_file);

    match codic_run(&path).await {
        // 正常に実行されたら翻訳されたテキストを表示
        Ok(text) => println!("{}", text),

        Err(error) => {
            match error {
                // サブコマンドが選択されたら
                Error::ChosenEditConfig{command} => {
                    match command {
                        // 新しいファイルを生成
                        error::EditConfig::Make =>  {
                            Config::make_default_file(&path).await.unwrap();
                            println!("{} を作成します。tokenにCodic APIキーを入力してください\n{}",
                                config_file, "codic config edit -t <token>");
                        },
                        error::EditConfig::Show => {
                            let config = Config::from_file(&path).unwrap();
                            println!("{:?}", config);
                        },
                        error::EditConfig::Edit(token, casing) => {
                            Config::edit(&path, token.as_deref(), casing.as_deref()).await.unwrap();
                            println!("編集しました。");
                            let config = Config::from_file(&path).unwrap();
                            println!("{:?}", config);
                        },
                        _ => (),
                    }
                },

                // コンフィグファイルが開けなかったら
                Error::CannotOpenConfig{error} => {
                    println!("{} オープンエラー", config_file);
                    println!("{}", error);
                    Config::make_default_file(&path).await.unwrap();
                    println!("{} を作成します。tokenにCodic APIキーを入力してください\n{}",
                                config_file, "codic config edit -t <token>");
                },

                // コンフィグファイルが破損していたら
                Error::ConfigSyntaxError{error} => {
                    println!("{} コンフィグファイル構文エラー", config_file);
                    println!("{}", error);
                },

                // CodicAPIに接続できなかったら
                Error::CouldNotConnect{error} => {
                    println!("接続エラー");
                    println!("{}", error);
                },

                // APIへのリクエストに問題があったら
                Error::CodicError(error) => {
                    println!("リクエストエラー");
                    println!("{}", error);
                },
            }
        }
    };
}
