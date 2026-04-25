# 章末総合演習 — 1〜8章の学習を活かした init.lua を書き上げる

**難易度**: ★★★  
**目安時間**: 30〜40分

---

## 課題

`exercise/init.lua` は空ファイルである。
1〜8章で学んだすべての操作を快適にするための `init.lua` を、自分の手で書き上げよ。

---

## 実装すべき項目

### 基本設定（ミッション1）

- 行番号・相対行番号
- Tab = スペース4個
- 検索設定（インクリメンタル・スマートケース）
- カーソル行ハイライト
- サインカラム常時表示（`signcolumn = "yes"`）
- `updatetime = 250`

### キーマッピング（ミッション2）

- leader キー = `<Space>`
- 検索ハイライト消去（`<Esc>`）
- 保存・終了（`<leader>w`・`<leader>q`）
- ウィンドウ移動（`<C-h/j/k/l>`）
- ターミナルモード終了（`<C-n>`）
- netrw 起動（`<leader>e`）
- quickfix ナビゲーション（`<leader>cn`・`<leader>cp`）

### LSP 設定（ミッション3）

- `LspAttach` でバッファローカルキーマップ
- rust-analyzer 起動
- 診断設定

### 追加チャレンジ（任意）

- `BufWritePre` で Rust ファイルの自動フォーマット
- `TermOpen` でターミナルバッファの行番号を無効化
- `colorscheme` の設定（`habamax` など組み込みカラースキームを使う）

---

## 制約

- プラグインマネージャは使わない（`vim.opt`・`vim.keymap`・`vim.api` のみ）
- ファイルを書き終えたら `:source %` で反映し、実際に動作することを確認すること

## ゴール

`goal/init.lua` と「機能的に同等」であること（書き方は自分流でよい）。
具体的には:
- `gd` で Rust の定義にジャンプできる
- `<leader>w` でファイルが保存できる
- `cargo check` がエラーを出すと診断として表示される

---

## 自己評価

完成したら以下を確認:

```
:checkhealth        ← LSP・provider の状態を確認
:verbose map gd     ← gd がどのファイルで定義されているか確認
:lua print(vim.inspect(vim.lsp.get_active_clients()))  ← LSP クライアント確認
```
