# ミッション 2 — s/S/c/C で範囲を書き直す

**難易度**: ★★☆  
**目安時間**: 5〜10分

---

## 課題

`exercise.rs` のコードを、`s`・`S`・`c{motion}`・`C` を使って書き直せ。
「削除してインサートモードへ」という動作を意識しながら最小手数で変更する。

1. `let result = vec![];` の行を丸ごと `let result: Vec<String> = Vec::new();` に書き直す
2. `return Err("invalid input");` の `"invalid input"` 部分を `"入力値が不正です"` に変更する
3. `fn process(data: Vec<u8>)` の引数部分 `data: Vec<u8>` を `input: &[u8]` に変更する
4. `    println!("done");` の行を空行に変更する（行の内容を消してインサートモードに入り、即 Esc）
5. `let x = calculate();` の `calculate()` をカーソルから行末まで `compute_value(input)?` に変更する

## 制約

- 課題1: `S` で行全体を置き換える
- 課題2: `ci"` でクォート内を変更
- 課題3: `ct)` で `)` の前まで変更
- 課題4: `S` → `<Esc>` で行を空にする
- 課題5: `C` でカーソルから行末まで変更

## ゴール

`goal.rs` と同一の状態にすること。

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- `S` は現在行を削除してインサートモードに入る（インデントは自動）
- `ci"` = change inside double-quotes
- `ct)` = change till `)` (`)` は含まない)
- `C` = `c$` と同じ（カーソルから行末を変更）
- `s` は1文字だけ変えたいときに使う（`cl` と同じだが1キー短い）

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```
1. /let result → S → let result: Vec<String> = Vec::new();<Esc>
2. /invalid    → ci" → 入力値が不正です<Esc>
3. /fn process → f( → a → <Del>... （または ct) → input: &[u8]）
4. /println    → S → <Esc>
5. /let x      → f= → ww → C → compute_value(input)?<Esc>
```

</details>

---

## 別解

<details>
<summary>別解を見る（上達したら試してみよう）</summary>

- 課題3は `di(` でまず括弧内を削除 → `i` で挿入モード → `input: &[u8]` と入力
- `cit` (change inside tag) はHTMLやXMLで有効

</details>
