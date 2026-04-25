# ミッション 1 — netrw でプロジェクトを探索しバッファを管理する

**難易度**: ★☆☆  
**目安時間**: 5〜10分

---

## 前提

このミッションは `mission-1/project/` ディレクトリに小さな Rust プロジェクトが置いてある。
`nvim` を `mission-1/` で起動し、netrw から操作を始めること。

---

## 課題

1. `:Ex` で netrw を開き、`project/src/` ディレクトリに移動して `main.rs` を開く
2. `main.rs` を開いたまま `:Ex` で netrw に戻り、`project/src/lib.rs` も開く
3. `:ls` でバッファ一覧を確認し、バッファ番号で `main.rs` と `lib.rs` を切り替える
4. `Ctrl+^` で直前のバッファに素早く切り替える（交互切り替えを3回繰り返す）
5. `lib.rs` に `pub fn greet(name: &str) -> String` 関数を追加して保存する
6. `:bd` で `lib.rs` バッファを削除し、残りのバッファを `:ls` で確認する

## 制約

- ファイルを開くのは netrw または `:e` コマンドのみ（`:vsp`・`:sp` は次のミッションで使う）
- マウス禁止

## ゴール

`project/src/lib.rs` に `pub fn greet` 関数が追加されていること。

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- netrw で `Enter` はファイルを開く、`-` は親ディレクトリへ
- `:ls` の出力で `%` は現在のバッファ、`#` は直前のバッファ
- `Ctrl+^` = `:b#` と同等（`#` バッファへ切り替え）
- `:b main` のようにファイル名の一部でも補完して切り替えられる

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```
:Ex
（netrw で project/ → src/ → main.rs を Enter で開く）
:Ex
（lib.rs を Enter で開く）
:ls
:b1  （main.rs へ）
:b2  （lib.rs へ）
Ctrl+^  Ctrl+^  Ctrl+^
（lib.rs で関数を追加）
:w
:bd
:ls
```

</details>
