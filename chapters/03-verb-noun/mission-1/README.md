# ミッション 1 — d/c/y + テキストオブジェクトで基本編集

**難易度**: ★☆☆  
**目安時間**: 5〜10分

---

## 課題

`exercise.rs` を開き、operator + text object の組み合わせだけで以下の編集を行え。

1. `"error: connection failed"` → `"接続に失敗しました"` に変更する（`ci"` を使う）
2. `vec![1, 2, 3, 4, 5]` の `[1, 2, 3, 4, 5]` 部分をヤンクして別の行に貼り付ける（`ya[` を使う）
3. `if let Some(v) = result {` の `result` を `value` に変更する（`ciw` を使う）
4. `fn compute(x: i32, y: i32) -> i32` の引数 `x: i32, y: i32` を削除する（`di(` を使う）
5. `{ return Err(e); }` のブロック全体（波括弧含む）を `{ return Ok(()); }` に変更する（`ca{` を使う）

## 制約

- 各課題で指定された text object コマンドを使うこと
- 矢印キー・マウス禁止

## ゴール

`goal.rs` と同一の状態にすること。

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- `i"` = ダブルクォートの内側（クォート自体は含まない）
- `a"` = ダブルクォートを含む範囲
- `i(` = 丸括弧の内側
- `a{` = 波括弧を含む範囲
- `iw` = 単語（カーソルが単語上にあれば自動的に選択）

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```
1. /connection → ci" → 接続に失敗しました<Esc>
2. /vec!       → f[ → ya[ → 別の行へ移動 → p
3. /result     → ciw → value<Esc>
4. /fn compute → f( → di(
5. /return Err → f{ → ca{ → { return Ok(()); }<Esc>
```

</details>

---

## 別解

<details>
<summary>別解を見る（上達したら試してみよう）</summary>

- 課題3は `*` でカーソル下の `result` を検索 → `cgn` で最初のマッチを変更 → `.` で繰り返し
- `di(` の後に `p` で括弧内の内容を別の場所に移動できる
- `yib` と `ya(` は `yi(` と `ya(` の別名

</details>

---

## 演習リセット

演習で `exercise.rs` を編集した後、コミット前に元の状態へ戻すこと。
そうしないと次回の演習で「最初から `goal.rs` と同じ」になってしまう。

```bash
# このミッションだけリセット
scripts/reset.sh 03-verb-noun/mission-1

# 全ミッションをまとめてリセット
scripts/reset.sh
```
