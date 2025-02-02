# DNCL → Rust トランスパイラ

DNCL (大学入試センター言語, Daigaku Nyushi Center Language) をRustにトランスパイルするマクロです。

## DNCLとは

日本の大学入試共通テスト (旧センター試験) 情報 において用いられる疑似プログラミング言語です。

[共通テスト手順記述標準言語 (DNCL) の説明](https://www.dnc.ac.jp/albums/abm.php?d=67&f=abm00000819.pdf&n=R4_%E5%85%B1%E9%80%9A%E3%83%86%E3%82%B9%E3%83%88%E6%89%8B%E9%A0%86%E8%A8%98%E8%BF%B0%E6%A8%99%E6%BA%96%E8%A8%80%E8%AA%9E%EF%BC%88DNCL%EF%BC%89%E3%81%AE%E8%AA%AC%E6%98%8E.pdf)

仕様は変更がなされるようで、本クレートは以下のバージョンに基づいています。

- [令和７年度試験の問題作成の方向性、試作問題等 | 独立行政法人 大学入試センター](https://www.dnc.ac.jp/kyotsu/shiken_jouhou/r7/r7_kentoujoukyou/r7mondai.html)
  - [試作問題「情報」の概要](https://www.dnc.ac.jp/albums/abm.php?d=511&f=abm00003141.pdf&n=6-1_%E6%A6%82%E8%A6%81%E3%80%8C%E6%83%85%E5%A0%B1%E3%80%8D.pdf)

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
