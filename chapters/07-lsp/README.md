# 第7章 LSP

## 概要

nvim 0.10 の組み込み LSP を使い、IDEと同等のコード支援をプラグインなしで実現する。
`rust-analyzer` を例に、定義ジャンプ・診断・フォーマット・コードアクションを習得する。

## 前提

`rust-analyzer` のインストールが必要:

```bash
# rustup 経由（推奨）
rustup component add rust-analyzer

# または直接ダウンロード
# https://github.com/rust-lang/rust-analyzer/releases
```

LSP の起動設定は [第9章](../09-init-lua/) で扱う。本章の演習は `init.lua` に最低限の設定済みであることを前提とする。

---

## コマンド早見表

### ナビゲーション

| コマンド | 由来 | 動作 |
|----------|------|------|
| `gd` | **g**o to **d**efinition | 定義元へジャンプ |
| `gD` | **g**o to **D**eclaration | 宣言へジャンプ |
| `gr` | **g**o to **r**eferences | 参照一覧を quickfix に表示 |
| `gI` | **g**o to **I**mplementation | 実装へジャンプ |
| `Ctrl+o` | **o**lder（古い位置） | ジャンプ前の位置に戻る |
| `Ctrl+t` | **t**ag stack | タグスタックを戻る |

### 情報表示

| コマンド | 由来 | 動作 |
|----------|------|------|
| `K` | **K**eyword help（man ページ参照の流用） | カーソル下シンボルのドキュメント表示 |
| `Ctrl+k` | （関数シグネチャ）| シグネチャヘルプを表示 |
| `[d` | 前の **d**iagnostic | 前の診断（エラー・警告）へ |
| `]d` | 次の **d**iagnostic | 次の診断へ |
| `:lua vim.diagnostic.open_float()` | **diagnostic** + フロート表示 | 現在行の診断を表示 |
| `:lua vim.diagnostic.setqflist()` | quickfix に出力 | 全診断を quickfix に出力 |

### 編集支援

| コマンド | 由来 | 動作 |
|----------|------|------|
| `:lua vim.lsp.buf.format()` | **format** | ファイルをフォーマット |
| `:lua vim.lsp.buf.rename()` | **rename** | シンボルをリネーム |
| `:lua vim.lsp.buf.code_action()` | **code action** | コードアクションを表示 |
| `Ctrl+x Ctrl+o` | **x** モード + **o**mni（全方位補完）| オムニ補完（LSP ベース） |

### LSP の状態確認

| コマンド | 由来 | 動作 |
|----------|------|------|
| `:lua vim.lsp.get_active_clients()` | active clients | アクティブな LSP クライアント一覧 |
| `:LspInfo` | **LSP info** | LSP の接続状態（nvim 0.10+） |
| `:lua vim.lsp.buf.hover()` | **hover** | `K` と同等（hover ドキュメント） |

### 推奨キーマッピング（第9章で設定）

```lua
vim.keymap.set('n', 'gd',         vim.lsp.buf.definition)
vim.keymap.set('n', 'gr',         vim.lsp.buf.references)
vim.keymap.set('n', 'K',          vim.lsp.buf.hover)
vim.keymap.set('n', '<leader>rn', vim.lsp.buf.rename)
vim.keymap.set('n', '<leader>ca', vim.lsp.buf.code_action)
vim.keymap.set('n', ']d',         vim.diagnostic.goto_next)
vim.keymap.set('n', '[d',         vim.diagnostic.goto_prev)
```

---

## ミッション一覧

| ミッション | 内容 | 難易度 |
|-----------|------|-------|
| [ミッション 1](mission-1/README.md) | gd/gr/K でコードを読み解く | ★☆☆ |
| [ミッション 2](mission-2/README.md) | 診断を使ってコンパイルエラーを修正する | ★★☆ |
| [ミッション 3](mission-3/README.md) | rename・code action・format を活用したリファクタ | ★★★ |
| [章末総合演習](summary/README.md) | LSP フル活用で未知のコードを読んで改修する | ★★★ |
