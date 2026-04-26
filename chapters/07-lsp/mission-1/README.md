# ミッション 1 — gd/gr/K でコードを読み解く

**難易度**: ★☆☆  
**目安時間**: 5〜10分

---

## 前提

- `rust-analyzer` がインストールされていること
- `init.lua` に LSP の起動設定があること（[第9章](../../../09-init-lua/)参照）

---

## 課題

`exercise.rs` を開き、LSP のナビゲーション機能だけを使ってコードを読む。

1. `process_request` 関数の呼び出し箇所にカーソルを置き、`gd` で定義に飛ぶ
2. 定義を読んだ後 `Ctrl+o` で呼び出し元に戻る
3. `Handler` 構造体にカーソルを置き、`gr` で全参照箇所を quickfix に表示する
4. quickfix を `:copen` で開き、各参照箇所を確認する
5. `Error` 型にカーソルを置き、`K` でドキュメントを表示する
6. `AppError` の `from` 実装にカーソルを置き、`gI` で実装元へジャンプする

## 制約

- `/` 検索でのジャンプ禁止（LSP のコマンドのみ使う）
- quickfix は `:copen` / `:cn` / `:cp` で操作する

## ゴール

以下の問いに答えられること（紙やファイルに記録してもよい）:
- `process_request` の引数の型は何か？
- `Handler` は何箇所から参照されているか？
- `AppError` の `Display` 実装のメッセージは何か？

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- `gd` がキーマップされていない場合: `:lua vim.lsp.buf.definition()`
- `gr` がキーマップされていない場合: `:lua vim.lsp.buf.references()`
- `K` がキーマップされていない場合: `:lua vim.lsp.buf.hover()`
- LSP が起動しているか確認: `:LspInfo` または `:lua print(vim.inspect(vim.lsp.get_active_clients()))`

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```
/process_request → gd → （定義を確認） → Ctrl+o
/Handler → gr → :copen → :cn → :cn ...
/Error → K
/AppError → f f → （from 実装で） gI
```

</details>

---

## Rust 背景（読み飛ばし可）

このミッションの題材コードに出てくる Rust 構文の最低限のメモ。
LSP 操作だけに集中したい場合は飛ばしてよい。

- `Result<T, E>` — 成功 (`Ok(T)`) か失敗 (`Err(E)`) かを表す型。`?` 演算子は「`Err` なら即 return、`Ok` なら中身を取り出す」糖衣構文
- `Option<T>` — 値あり (`Some(T)`) か無し (`None`) を表す型。`ok_or_else(|| err)` は `None` を `Err(err)` に変換する
- トレイト実装 (`impl Trait for Type { ... }`) — Type に Trait の機能を追加する。`From<T>` を実装すると `T::into()` が使える
- `HashMap<K, V>` — キー → 値の辞書型。`.get(&key)` は `Option<&V>` を返す

LSP の `gd` / `gr` / `K` はこれらの型・関数の **定義位置や型情報を見るためのもの**。
コードの意味（何をしているか）は LSP では教えてくれないので、上記の概念は別途理解が必要。

---

## 演習リセット

演習で `exercise.rs` を編集した後、コミット前に元の状態へ戻すこと。
そうしないと次回の演習で「最初から `goal.rs` と同じ」になってしまう。

```bash
# このミッションだけリセット
scripts/reset.sh 07-lsp/mission-1

# 全ミッションをまとめてリセット
scripts/reset.sh
```
