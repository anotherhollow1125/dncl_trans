use cache::{cache_result, hash_content, load_cache};
use macro_::IntoSynRes;
use proc_macro2::TokenStream;
use query::QuerySetting;

mod cache;
mod macro_;
mod markdown;
mod query;

pub use macro_::MacroInput;
use syn::spanned::Spanned;

pub fn dncl_impl(input: MacroInput) -> syn::Result<TokenStream> {
    let res = transpile(input)?;

    Ok(file_content2token_stream(&res))
}

fn file_content2token_stream(res_code: &str) -> TokenStream {
    let codes = markdown::extract_rust_codes(res_code);

    let res_code = match codes.len() {
        0 => res_code.to_string(),
        _ => codes.join("\n"),
    };

    match res_code.parse() {
        Ok(ok) => ok,
        Err(_) => quote::quote! { compile_error!(#res_code); },
    }
}

const DEFAULT_MODEL: &str = "gpt-4o";
const DNCL_SPEC: &str = r#"大学入試共通テスト用プログラミング言語DNCLの仕様を以下に示します。

---
高等学校の「情報Ⅰ」の授業で使用するプログラミング言語は多様であることから、共通テスト『情報Ⅰ』の試作問題作成にあたり、共通テスト用のプログラム表記を使用します。以下、参考のためにその基本を例示します。しかしながら、問題文の記述を簡潔にするなどの理由で、この説明文書の記述内容に従わない形式で出題することもあります。したがって、共通テスト『情報Ⅰ』の受験に際しては、当該問題文の説明や指示に注意し、それらに沿って解答してください。なお、経過措置問題『旧情報（仮）』についても同様に扱うこととします。

# 1. 変数

通常の変数例: `kosu`, `kingaku_kei`

(変数名は英字で始まる英数字と `_` の並び)

配列変数の例: `Tokuten[3]`, `Data[2, 4]` (配列名は先頭文字が大文字)

※ 特に説明がない場合、配列の要素を指定する添字は `0` から始まる

# 2. 文字列

文字列はダブルクォーテーション `"` で囲む

```dncl
moji = "I'll be back.";
message = "祇園精舎の" + "鐘の声" # `+`で連結できる;
```

コード中に示した通り、 `+`で連結できる

# 3. 代入文

```dncl
kosu = 3, kingaku = 300 # 複数文を1行で表記できる
kingaku_goukei = kingaku * kosu
namae = "Komaba"
Data = [10, 20, 30, 40, 50, 60]
Tokentenのすべての値を0にする
nyuryoku = {外部からの入力}
```

# 4. 算術演算

加減剰余の四則演算は、 `+` 、 `-` 、 `*` 、 `/` で表す
整数の除算では、商(整数)を `÷` または `div` で、余りを `%` で表す
べき乗は `**` で表す

# 5. 比較演算

`==` (等しい)、 `!=` (等しくない)、 `>` 、 `<`、 `>=` 、 `<=`

# 6. 論理演算

`and` (論理積)、 `or` (論理和)、 `not` (否定)

# 7. 関数

## 値を返す関数を使用する例

```dncl
kazu = 要素数(Data)
saikoro = 整数(乱数() * 6) + 1
```

## 値を返さない関数を呼び出す例

```dncl
表示する(Data)
表示する(Kamoku[i], "の特典は", Tensu[i], "です")
```

※ 「表示する」関数はカンマ区切りで文字列や数値を連結できる
※ 「表示する」関数以外は基本的に問題中に説明あり (ない場合は関数名より忖度してください)

# 8. 制御文 (条件分岐)

```dncl
もし x < 3 ならば:
│ x = x + 1
└ y = y + 1
```

```dncl
もし x == 3 ならば:
|  x = x - 1
そうでなければ:
|= y = y * 2
```

```dncl
もし x >= 3 ならば:
|  x = x - 1
そうでなくもし x < 0 ならば:
|  x = x * 2
そうでなければ:
|= y = y * 2
```

※ `│` (または `|` )と `└` (または `|=` )で制御範囲を表し、 `└` (または `|=` ) の行は制御文の終わりの行を示す

# 9. 制御文 (繰り返し)

```dncl
x を 0 から 9 まで 1 ずつ増やしながら繰り返す:
└ goukei = goukei + Data[x]
```

※ `減らしながら` もある

```dncl
n < 10 の間繰り返す:
|  goukei = goukei + n
|= n = n + 1
```

※ `│` (または `|` )と `└` (または `|=` )で制御範囲を表し、 `└` (または `|=` ) の行は制御文の終わりの行を示す

# 10. コメント

```dncl
atai = 乱数() # 0 以上 1 未満のランダムな少数を atai に代入する
```

※ 1行内において # 以降の記述は処理の対象とならない

# 11. 補足

DNCLの仕様ではありませんが、トランスパイルの都合上入力が特殊になっていることがあります。以下の点に注意してください。

- 行の先頭に `(1)` や `（1）` のように行番号があることがありますが、この番号は単に無視してください。
- 余分な改行が入っていることがあります。もし改行が2行連続していてもそれは1つの改行区切りとして扱ってください。
- 空白による区切りが適切でない(多かったりまったくなかったりする)場合がありますが、本仕様において空白区切りによる曖昧さは発生しないはずです。いい感じに解析してください。

---

次にDNCLのプログラムが与えられますので、エントリポイントとなる `main` 関数を含めたRustプログラムへトランスパイルしてください。

なお、 `rand` 等のサードパーティクレートはユーザー側が自分で `Cargo.toml` に追加するため、存在するものと仮定して構いません。
"#;

fn transpile(
    MacroInput {
        model,
        seed,
        max_completion_tokens,
        prompt,
    }: MacroInput,
) -> syn::Result<String> {
    let span = prompt.span();
    let prompt = prompt.to_string().replace(";", "\n");
    let prompt = format!("```dncl\n{}\n```", prompt);

    if let Some(cache) = load_cache(&prompt) {
        return Ok(cache);
    }

    dotenvy::dotenv().ok();
    let api_key = std::env::var("OPENAI_API_KEY").into_syn(span)?;

    let hash = hash_content(&prompt);

    let setting = QuerySetting {
        api_key: api_key.as_str(),
        model: model.as_deref().unwrap_or(DEFAULT_MODEL),
        seed: seed.unwrap_or(hash),
        max_completion_tokens,
    };

    if cfg!(test) {
        dbg!(&[DNCL_SPEC, &prompt]);
    }

    let response = setting.query(&[DNCL_SPEC, &prompt]).into_syn(span)?;

    cache_result(&prompt, &response);

    Ok(response)
}

#[cfg(test)]
mod test {
    use super::transpile;
    use super::MacroInput;

    impl From<String> for MacroInput {
        fn from(value: String) -> Self {
            MacroInput {
                model: None,
                seed: None,
                max_completion_tokens: None,
                prompt: value.parse().unwrap(),
            }
        }
    }

    #[test]
    fn test_1() {
        let code = r#"
表示する("Hello, world!");
"#;

        let macro_input = MacroInput {
            model: Some("o1-preview".to_string()),
            ..MacroInput::from(code.to_string())
        };

        let res = transpile(macro_input).unwrap();

        dbg!(res);
    }

    #[test]
    fn test_2() {
        let code = r#"
Data=[3,18,29,33,48,52,62,77,89,97];
kazu=要素数(Data);
表示する("0~99の数字を入力してください");
atai={外部からの入力};
hidari=0, migi=kazu-1;
owari=0;
hidari <= migi and owari == 0 の間繰り返す: ;
|  aida=(hidari+migi) div 2 # 演算子 div は商の整数値を返す;
|  もし Data[aida] == atai ならば: ;
|  |  表示する(atai,"は",aida,"番目にありました");
|  |  owari=1;
|  そうでなくもし Data[aida]<atai ならば: ;
|  |  hidari=aida+1;
|  そうでなければ: ;
|= |= migi=aida-1;
もし owari==0 ならば: ;
|= 表示する(atai,"は見つかりませんでした");
表示する("添字"," ","要素");
iを0からkazu-1まで1ずつ増やしながら繰り返す: ;
|= 表示する(i," ",Data[i]);
"#;

        let macro_input = MacroInput {
            model: Some("o1-preview".to_string()),
            ..MacroInput::from(code.to_string())
        };

        let res = transpile(macro_input).unwrap();

        dbg!(res);
    }
}
