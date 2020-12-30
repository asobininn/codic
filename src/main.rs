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
                            match Config::make_default_file(&path).await {
                                Ok(_) => println!("{} を作成しました。tokenにCodic APIキーを入力してください、\n{}",
                                    config_file, "codic config edit -t <token>"),
                                Err(_) => println!("{}の作成に失敗しました。", path.to_string_lossy()),
                            };
                        },
                        // ファイルを表示
                        error::EditConfig::Show => {
                            println!("{:?}", Config::from_file(&path).expect("ファイルの表示に失敗しました"));
                        },
                        // ファイルを削除
                        error::EditConfig::Remove => {
                            match std::fs::remove_file(&path) {
                                Ok(_) => println!("{}を削除しました。", path.to_string_lossy()),
                                Err(_) => eprintln!("{}の削除に失敗しました。", path.to_string_lossy()),
                            };
                        },
                        // ファイルを編集
                        error::EditConfig::Edit(token, casing) => {
                            match Config::edit(&path, token.as_deref(), casing.as_deref()).await {
                                Ok(_) => {
                                    println!("編集しました。");
                                    println!("{:?}", Config::from_file(&path).unwrap());
                                },
                                Err(_) => println!("{}の編集に失敗しました。", path.to_string_lossy()),
                            };
                        },
                        error::EditConfig::None => (),
                    }
                },

                // コンフィグファイルが開けなかったら
                Error::CannotOpenConfig{error} => {
                    println!("{} オープンエラー", config_file);
                    println!("{}", error);
                    match Config::make_default_file(&path).await {
                        Ok(_) => println!("{} を作成しました。tokenにCodic APIキーを入力してください、\n{}",
                            config_file, "codic config edit -t <token>"),
                        Err(_) => println!("{}の作成に失敗しました。", path.to_string_lossy()),
                    };
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
