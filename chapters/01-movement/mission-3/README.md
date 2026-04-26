# ミッション 3 — マーク・ジャンプリストで複数箇所を往復編集

**難易度**: ★★★  
**目安時間**: 10分

---

## 課題

`exercise.rs` は3箇所に関連する変更が必要な Rust コードである。
マークとジャンプリストを活用して、3箇所を効率よく往復しながら編集せよ。

1. `struct AppConfig` の `db_url` フィールドに `/// データベース接続URL` のドキュメントコメントを追加する
2. `fn connect` 関数で `db_url` を使っている行を `&self.config.db_url` から `self.config.db_url.as_str()` に変更する
3. `fn validate` 関数で `db_url.is_empty()` を `db_url.is_empty() || db_url.len() < 10` に変更する

## 制約

- 3箇所の間を往復する際は `m{a}`・`` `{a} `` または `Ctrl+o`/`Ctrl+i` を使うこと
- 毎回 `/` 検索でジャンプするのは禁止

## ゴール

`goal.rs` と同一の状態にすること。

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- まず3箇所にマークを設定してから編集すると行き来が楽
  - `ma` → 1箇所目にマーク a
  - `mb` → 2箇所目にマーク b
  - `mc` → 3箇所目にマーク c
  - `` `a `` / `` `b `` / `` `c `` でそれぞれへジャンプ
- `Ctrl+o` でジャンプリストを遡ると「さっきいた場所」に戻れる
- `O` コマンドで `struct` フィールドの上に行を挿入してコメントを書く

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```
1. /db_url    → struct 内の db_url を検索
   ma         → マーク a を設定
   O          → 上に新行を挿入
   /// データベース接続URL<Esc>

2. /fn connect → connect 関数へ
   mb          → マーク b を設定
   /db_url<CR> → db_url の行へ
   f.          → '.' へジャンプ
   ct)         → ')' の手前まで変更
   .as_str()<Esc>

3. /fn validate → validate 関数へ
   mc           → マーク c を設定
   /is_empty    → is_empty の行へ
   f)           → ')' へジャンプ
   a || db_url.len() < 10<Esc>
```

</details>

---

## 別解

<details>
<summary>別解を見る（上達したら試してみよう）</summary>

- グローバルマーク（大文字）`mA`、`mB`、`mC` を使うとファイルを閉じても位置が保持される
- `:marks` コマンドで現在設定されているマーク一覧を確認できる
- `''`（シングルクォート2回）で「直前のジャンプ位置」に戻れる

</details>

---

## 演習リセット

演習で `exercise.rs` を編集した後、コミット前に元の状態へ戻すこと。
そうしないと次回の演習で「最初から `goal.rs` と同じ」になってしまう。

```bash
# このミッションだけリセット
scripts/reset.sh 01-movement/mission-3

# 全ミッションをまとめてリセット
scripts/reset.sh
```
