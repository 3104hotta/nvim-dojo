# ミッション 1 — i/a/o/O/I/A を使い分けて挿入

**難易度**: ★☆☆  
**目安時間**: 5〜10分

---

## 課題

`exercise.rs` を開き、以下の編集を「最適なモードエントリコマンド」で行え。
毎回 `i` でインサートモードに入ってカーソルを動かすのは禁止。

1. `struct Server` の `}` の直前（`port: u16` の行の下）に `timeout: u64,` フィールドを追加する
2. `fn start` の行頭（`fn` の前）に `pub ` を追加する
3. `let addr =` の行末（セミコロンの後）に `  // bind address` コメントを追加する
4. `// TODO` で始まる行の下に新しい `// FIXME: not implemented` 行を追加する
5. `"0.0.0.0"` の文字列を `"127.0.0.1"` に変更する

## 制約

- 各操作で使うべきコマンドを選ぶこと:
  - 行末への追記 → `A`
  - 行頭への挿入 → `I`
  - 下に新行 → `o`
  - 上に新行 → `O`
  - 行内の特定位置 → `f`/`t` + `i`/`a`/`s`

## ゴール

`goal.rs` と同一の状態にすること。

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- `port: u16,` の行で `o` → 下に新行を作ってフィールドを追加
- `fn start` の行で `I` → 行頭からインサートモード開始
- 行末コメントは `A` → スペース2つ + `// bind address`
- `// TODO` の行で `o` → 下に新行を追加
- `ci"` でクォート内を変更

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```
1. /port: u16 → o → timeout: u64,<Esc>
2. /fn start  → I → pub <Esc>
3. /let addr  → A →   // bind address<Esc>
4. /TODO      → o → // FIXME: not implemented<Esc>
5. /0.0.0.0   → ci" → 127.0.0.1<Esc>
```

</details>

---

## 別解

<details>
<summary>別解を見る（上達したら試してみよう）</summary>

- 課題5は `:%s/"0.0.0.0"/"127.0.0.1"/g` で一括置換
- 課題2は `0i` でも行頭インサートできるが、`I` の方が1キー短い
- `A` のあと `Ctrl+o` でノーマルコマンドを1回実行してから戻れる

</details>
