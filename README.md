# DNCL → Rust トランスパイラ

DNCL (大学入試センター言語, Daigaku Nyushi Center Language) をRustにトランスパイルするマクロです。

## Example

```rust:Rust
dncl_trans::dncl!(
    @model = "gpt-4o";
    @seed = 123456;
    @editing = false;

    r#"
# 本プログラムでは配列の添字は 1 から始まります
Akibi = [5, 3, 4]
buinsu = 3
tantou = 1
buin を 2 から buinsu まで 1 ずつ増やしながら繰り返す:
│  もし Akibi[buin] < Akibi[tantou] ならば:
└  └ tantou = buin
表示する("次の工芸品の担当は部員", tantou, "です。")
"#
);
```

↓ 展開後

```rust:Rust
fn main() {
    let mut akibi = [5, 3, 4];
    let buinsu = 3;
    let mut tantou = 1;
    
    for buin in 2..=buinsu {
        if akibi[(buin - 1) as usize] < akibi[(tantou - 1) as usize] {
            tantou = buin;
        }
    }
    
    println!("次の工芸品の担当は部員{}です。", tantou);
}
```

カラクリは単純で、[OpenAI API](https://platform.openai.com/) に与えられたDNCLコードを送り、トランスパイルしてもらった結果で置換しています。

## 準備

OpenAI APIを叩くために[APIキー](https://platform.openai.com/api-keys)が必要です。APIキーの取得後、 `.env` ファイルに以下のように設定してください。

```plain:.env
OPENAI_API_KEY="取得したAPIキー"
```

あるいはコンパイル時に直接環境変数として指定してもかまいません。

## 使い方

先頭に `@` で始まるオプション設定をし、最後に文字列リテラルあるいはそのまま直書きでDNCLソースコードを記載することでトランスパイルされます。

```rust
dncl_trans::dncl!(
    @model = "o1-preview";
    @max_completion_tokens = 4096;
    @seed = 123456;
    @editing = false; // 編集中はtrueにすることでAPIを叩きに行かないようにする
    // @file = "もしファイル分割しているならこの変数で指定.dncl";

    r#"
    /* ここにDNCL記法のコードを書く */
    "#
);
```

設定項目一覧です。ソースコード以外はすべてオプションで、省略可能です。

|設定項目|効果|
|:--|:--|
|`@model`| 使用するGPTのモデルを指定。デフォルトは `gpt-4o` 。 `o1-preview` などを指定可能 |
|`@max_completion_tokens`| 返答トークンの最大値を調整するために使用。返答が切れてしまった時などにここを調整して長くできる(かも) |
|`@seed`| シード値。出力が期待したものではなかった時、入力を変化させずに別な出力を試したい時に使用 |
|`@editing`| 編集中かどうかを表すフラグ。 `true` の間はAPIを叩きに行かなくなる。デフォルトは `false` |
|`@file`| 別なファイルにDNCLプログラムを記述したい時に使用。本変数指定時はその後のDNCL入力は読み込まない |
|`r#"..."#`|DNCLソースコード部分を文字列リテラルで指定|

DNCLソースコードは文字列リテラルで指定することを推奨します。(DNCLオリジナルの文法だとトークン木として不正になることがあるため)

```rust:文字列リテラルで利用
dncl_trans::dncl!(
    r#"
    res = 1 + 1
    表示する("1 + 1 = ", res)
    "#
);
```

そのまま記述する場合は、改行区切りの最後に `;` が必要です。

```rust:そのまま記述
dncl_trans::dncl!(
    res = 1 + 1;
    表示する("1 + 1 = ", res);
);
```

## キャッシュファイルについて

ChatGPTからの返答は `gpt_responses` ディレクトリに保存され、コードが変わらないうちはこちらのキャッシュがコンパイルに利用されます。もし望まない結果になったりエラーレスポンスが帰ってきた場合は、シード値を変えてみたり、キャッシュファイルを削除の上再コンパイルしてみてください。
