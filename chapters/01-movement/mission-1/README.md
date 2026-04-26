# ミッション 1 — f/t/w/b で狙った場所へ一直線

**難易度**: ★☆☆  
**目安時間**: 5〜10分

---

## 課題

`exercise.rs` を nvim で開き、以下の編集を **`hjkl` を使わずに** 行え。

1. `fn parse_config` の引数 `path: &str` の `path` を `file_path` にリネームする
2. `let timeout = 30;` の `30` を `60` に変更する
3. `// TODO: validate` のコメントを削除する
4. `"localhost"` を `"127.0.0.1"` に変更する

## 制約

- `h` / `j` / `k` / `l` による移動は禁止
- 各変更は `f`・`t`・`w`・`b`・`e`・`0`・`$` などの移動コマンドを使うこと

## ゴール

`goal.rs` と同一の状態にすること。

```
before: fn parse_config(path: &str, ...) {
after:  fn parse_config(file_path: &str, ...) {

before:     let timeout = 30;
after:      let timeout = 60;

before:     // TODO: validate
after:      （行ごと削除）

before:     host: "localhost".to_string(),
after:      host: "127.0.0.1".to_string(),
```

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- `fn` の行に行ったら `f p` で `path` の `p` へジャンプできる
- `ciw` で単語を変更するとインサートモードに入れる
- `f3` で `30` の `3` へジャンプ → `cw` で変更
- `//` のコメント行は `dd` で行ごと削除
- `f"` でクォートの手前へ、`ci"` でクォート内を変更

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```
1. fn 行にカーソルを置く
   fp      → 'p' の p へジャンプ
   ciw     → 単語変更モード
   file_path<Esc>

2. timeout 行へ移動（/timeout<Enter>）
   f3      → '3' へジャンプ
   cw      → 単語変更
   60<Esc>

3. TODO 行へ移動（/TODO<Enter>）
   dd      → 行削除

4. localhost 行へ移動（/localhost<Enter>）
   f"      → '"' へジャンプ
   ci"     → クォート内変更
   127.0.0.1<Esc>
```

</details>

---

## 別解

<details>
<summary>別解を見る（上達したら試してみよう）</summary>

- `*` でカーソル下の `path` を検索 → `cgn` で次のマッチを変更 → `.` で繰り返し
- `:%s/\<path\>/file_path/g` で一括置換（ただし引数以外の `path` も変わるので注意）

</details>

---

## 演習リセット

演習で `exercise.rs` を編集した後、コミット前に元の状態へ戻すこと。
そうしないと次回の演習で「最初から `goal.rs` と同じ」になってしまう。

```bash
# このミッションだけリセット
scripts/reset.sh 01-movement/mission-1

# 全ミッションをまとめてリセット
scripts/reset.sh
```
