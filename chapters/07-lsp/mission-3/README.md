# ミッション 3 — rename・code action・format を活用したリファクタ

**難易度**: ★★★  
**目安時間**: 15分

---

## 課題

`exercise.rs` に対して、LSP の高度な機能を使ってリファクタを行え。

1. `processData` 関数を `process_data` にリネームする（`<leader>rn` または `:lua vim.lsp.buf.rename()`）
   - リネームが全参照箇所に反映されることを確認する
2. `deprecated_method` にカーソルを置き、コードアクションを実行して推奨される代替に変更する
3. ファイル全体をフォーマットする（`<leader>f` または `:lua vim.lsp.buf.format()`）
4. `impl Display for AppError` の `fmt` メソッドにカーソルを置き、`K` でドキュメントを確認してから正しく実装する
5. 未実装の trait メソッドにカーソルを置き、code action で自動生成する

## 制約

- 課題1 は `:%s` による置換禁止。LSP の rename を使うこと
- 課題3 は手動でのスペース調整禁止。`format()` を使うこと

## ゴール

`goal.rs` と同一の状態（フォーマット済み・リネーム済み・trait 実装済み）になっていること。

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- `<leader>rn` が設定されていない場合: `:lua vim.lsp.buf.rename()`
- rename は入力プロンプトが出るので、新しい名前を入力して `Enter`
- code action は選択肢がポップアップで出る。矢印キーまたは番号で選択
- `rust-analyzer` の code action は "Fill match arms"・"Add missing impl items" など

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```
1. /processData → <leader>rn → process_data → Enter

2. /deprecated_method → <leader>ca → （選択肢から選ぶ）

3. :lua vim.lsp.buf.format()

4. /fmt → K → （ドキュメント確認） → 実装

5. /未実装メソッド → <leader>ca → "Implement missing members"
```

</details>

---

## Rust 背景（読み飛ばし可）

このミッションは LSP の rename / code action / format を使ったリファクタ練習。
題材で扱う Rust 概念の最低限のメモ。

- **トレイト** — 「型が満たすべきメソッドの契約」。例: `Display` トレイトを実装すると `println!("{}", value)` で表示できるようになる
- **`Display` の `fmt` メソッド** — シグネチャは `fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`。`'_` は無名ライフタイム、`Result` は成功/失敗を表す。`write!(f, "...", args)` マクロで `f` に書き出す
- **命名規約** — Rust では関数・変数は `snake_case`、型は `PascalCase`、定数は `SCREAMING_SNAKE_CASE`。LSP の rename はこれに沿った命名なら警告されない
- **`derive` マクロ** — `#[derive(Debug, Clone)]` で「`Debug` トレイトと `Clone` トレイトの実装を自動生成」する
- **code action "Add missing impl items"** — トレイトに含まれる未実装メソッドのシグネチャだけを生成してくれる（中身は `todo!()` になる）

LSP の `<leader>rn` (rename) や `<leader>ca` (code action) は **シグネチャ生成や名前の一括変更**を任せられる。
中身のロジック（`write!` で何を出力するか等）は手で書く必要がある点に注意。

---

## 演習リセット

演習で `exercise.rs` を編集した後、コミット前に元の状態へ戻すこと。
そうしないと次回の演習で「最初から `goal.rs` と同じ」になってしまう。

```bash
# このミッションだけリセット
scripts/reset.sh 07-lsp/mission-3

# 全ミッションをまとめてリセット
scripts/reset.sh
```
