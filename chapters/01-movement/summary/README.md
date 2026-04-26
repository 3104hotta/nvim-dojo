# 章末総合演習 — 移動コマンドを駆使してコードリーディング

**難易度**: ★★★  
**目安時間**: 15〜20分

---

## シナリオ

小規模な HTTP クライアントライブラリのコードレビューを任された。
`exercise.rs` を読み解きながら、指定された変更をすべて `hjkl` なしで完了させよ。

## 課題

以下の7項目を完了させること:

1. `struct HttpClient` の `base_url` フィールドを `base_url: String` から `base_url: url::Url` に変更する（型のみ変更、フィールド名は変えない）
2. ファイル先頭の `use` ブロックに `use url::Url;` を追加する
3. `fn new` 関数の引数 `base_url: &str` を `base_url: Url` に変更する
4. `fn get` 関数内の `format!("{}/{}",` の部分を `format!("{}/{}",` → `format!("{}{}", self.base_url,` に変更する
5. 最も行数の多い関数を探し、その関数名の行に `// NOTE: refactor candidate` コメントを追加する
6. `todo!()` マクロが含まれる行をすべて削除する（何行あるか数えてから削除する）
7. ファイル末尾に `// reviewed by: <your-name>` を追記する

## 制約

- `hjkl` 移動禁止
- 課題5の「最も行数の多い関数を探す」には `/fn ` 検索 + `n` で順に確認すること（`:grep` 禁止）
- 課題6の `todo!()` は何行あるか、先に検索でカウントしてから削除する

## ゴール

`goal.rs` と同一の状態にすること。

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- `%` で括弧の対応箇所へジャンプすると関数の終端を素早く見つけられる
- `/todo!` で全 `todo!()` を検索 → `n` で次へ → `dd` で削除 → `n.` で繰り返し
- `G` → `o` でファイル末尾に新行を追加
- `Ctrl+o` でジャンプリストを辿れば「さっきいた関数」に戻れる

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```
課題2: gg → use の最終行へ移動 → o → use url::Url;<Esc>
課題1: /base_url → cW → base_url: url::Url
課題3: /fn new → f& → cw → Url
課題6: /todo! → dd → n → dd → （繰り返し）
課題7: G → o → // reviewed by: <your-name><Esc>
```

</details>

---

## 振り返り

- 今回の課題で最も時間がかかった移動はどれか？
- `hjkl` を使いたくなった瞬間はあったか？そのときどのコマンドが代替になったか？
- 次回同じファイルを編集するとしたら、どの操作を改善できるか？

---

## 演習リセット

演習で `exercise.rs` を編集した後、コミット前に元の状態へ戻すこと。
そうしないと次回の演習で「最初から `goal.rs` と同じ」になってしまう。

```bash
# このミッションだけリセット
scripts/reset.sh 01-movement/summary

# 全ミッションをまとめてリセット
scripts/reset.sh
```
