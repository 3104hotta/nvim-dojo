# ミッション 1 — `:terminal` と `:!` で git 操作を nvim 内で完結

**難易度**: ★☆☆  
**目安時間**: 5〜10分

---

## 前提

`mission-1/project/` は git リポジトリになっている。

---

## 課題

nvim を終了せずに以下の git 操作をすべて完了せよ。

1. `:!git status` で現在の状態を確認する
2. `exercise.rs` を開き、`fn main` の中に `println!("Hello, nvim!");` を追加して保存する
3. `:!git diff %` で変更内容を確認する
4. `:!git add %` でステージングする
5. `:sp | terminal` でターミナルを開き、`git log --oneline -5` でログを確認する
6. ターミナルで `git commit -m "add hello message"` を実行する
7. `Ctrl+\ Ctrl+n` でターミナルモードを抜け、ウィンドウを閉じる（`Ctrl+w q`）

## 制約

- nvim を終了してターミナルで git を操作するのは禁止
- `:!` と `:terminal` を両方使うこと

## ゴール

コミットが作成されていること（`:!git log --oneline -1` で確認）。

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- `:!{cmd}` は出力を表示して終わる（インタラクティブ操作には向かない）
- `:terminal` はフルターミナルなので入力が必要な操作（コミットメッセージ入力など）に使う
- `:r !git log --oneline -3` で git ログの出力をカーソル位置に挿入できる

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```
:!git status
（exercise.rs を開いて編集・保存）
:!git diff %
:!git add %
:sp | terminal
i
git log --oneline -5
git commit -m "add hello message"
Ctrl+\ Ctrl+n
Ctrl+w q
:!git log --oneline -1   （確認）
```

</details>
