# ミッション 2 — vimdiff でファイルの差分を確認・マージ

**難易度**: ★★☆  
**目安時間**: 10分

---

## 課題

`mission-2/` には `before.rs` と `after.rs` の2バージョンがある。
vimdiff で差分を確認し、`before.rs` に `after.rs` の変更を取り込め。

**`after.rs` の変更内容**（事前に見ない方が練習になる）:
- 新しい関数が追加されている
- エラーハンドリングが改善されている
- 不要なコメントが削除されている

## 手順

1. ターミナルで `vimdiff before.rs after.rs` を実行して vimdiff を起動する（または nvim を開いてから `:e before.rs | vsp after.rs | windo diffthis`）
2. `]c` で最初の差分へジャンプする
3. 各差分を確認し、`before.rs` 側に取り込む変更は `dp` で（put）、残す変更は無視する
4. すべての差分を処理したら `:diffoff` で diff モードを終了する
5. `:w` で `before.rs` を保存する

## 制約

- マウスによる操作禁止
- `do`（diff obtain）と `dp`（diff put）を少なくとも1回ずつ使うこと

## ゴール

`before.rs` が `goal.rs` と同一の内容になっていること。

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- `]c` = 次の diff チャンクへ（`[c` = 前へ）
- `do` = 相手ウィンドウから変更を取得（obtain）: 相手側の変更を自分側に適用
- `dp` = 自分側の変更を相手ウィンドウへ送る（put）
- `Ctrl+w w` でウィンドウを切り替えながら `do`/`dp` を使い分ける
- `:diffupdate` で diff を再計算（編集後にズレた場合）

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```
（ターミナルで）
vimdiff before.rs after.rs

（vimdiff 内で）
]c          （最初の差分へ）
do          （after.rs の内容を before.rs に取り込む）
]c          （次の差分へ）
（差分を確認して do または無視）
... （繰り返し）
:diffoff
:w
```

</details>
