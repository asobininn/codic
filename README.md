# CodicAPIのバインディングプログラム
CodicAPIを使用し、日本語からそれらしい英語の変数名に翻訳します。

# 初期設定
1. CodicよりAPIキーを入手する<br>
codic: <https://codic.jp/>
2. コンフィグファイルの設定
```
codic config --make
codic config edit -t <APIキー>
```

# 使い方 例
```
codic こんにちは世界
codic こんにちは世界 --aA
codic こんにちは世界 --AA
codic こんにちは世界 --a_a
codic こんにちは世界 --A_A
codic こんにちは世界 --a-a
```

# コンフィグファイルの設定
場所は実行ファイルと同じディレクトリに存在します。<br>
.codic_config.json<br>
初期設定では、casingがcamelケースになっています。<br>

設定 例
```
// コンフィグファイルの表示
codic config --show
// デフォルトのコンフィグファイルを新規作成する
codic config --make
// コンフィグファイルの編集
codic config edit -t <APIキー> --c_A_A
```

削除
```
codic config remove
```

# License
MIT License
