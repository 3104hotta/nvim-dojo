# 第9章 init.lua

## 概要

プラグインなしで `init.lua` を書き、自分だけの nvim 環境を作る。
基本オプション・キーマッピング・組み込み LSP の起動設定を段階的に構築する。

---

## コマンド早見表

### 基本オプション（`vim.opt`）

```lua
vim.opt.number         = true    -- 行番号表示
vim.opt.relativenumber = true    -- 相対行番号
vim.opt.expandtab      = true    -- Tab をスペースに変換
vim.opt.tabstop        = 4       -- Tab 幅
vim.opt.shiftwidth     = 4       -- インデント幅
vim.opt.smartindent    = true    -- スマートインデント
vim.opt.wrap           = false   -- 折り返し無効
vim.opt.cursorline     = true    -- カーソル行をハイライト
vim.opt.hlsearch       = true    -- 検索結果をハイライト
vim.opt.incsearch      = true    -- インクリメンタル検索
vim.opt.ignorecase     = true    -- 大文字小文字を無視
vim.opt.smartcase      = true    -- 大文字を含む場合は区別
vim.opt.termguicolors  = true    -- True Color を有効化
vim.opt.signcolumn     = "yes"   -- サインカラムを常に表示（LSP 診断用）
vim.opt.updatetime     = 250     -- CursorHold の遅延（ms）
vim.opt.splitright     = true    -- vsp で右に開く
vim.opt.splitbelow     = true    -- sp で下に開く
```

### キーマッピング（`vim.keymap.set`）

```lua
-- 基本形
vim.keymap.set(mode, lhs, rhs, opts)
-- mode: 'n'(normal), 'i'(insert), 'v'(visual), 'x'(visual-only), 't'(terminal)

-- よく使うオプション
{ noremap = true, silent = true }
{ desc = "説明文" }   -- :map で確認できる説明

-- 例
vim.keymap.set('n', '<leader>w', ':w<CR>', { desc = "Save file" })
vim.keymap.set('n', '<Esc>', ':noh<CR>', { silent = true })
vim.keymap.set('t', '<C-n>', '<C-\\><C-n>', { desc = "Exit terminal mode" })
```

### leader キーの設定

```lua
vim.g.mapleader      = " "   -- スペースを leader に
vim.g.maplocalleader = "\\"  -- ローカル leader
-- ※ keymap.set より前に書く必要がある
```

### 組み込み LSP の起動（nvim 0.10+）

```lua
vim.lsp.start({
  name = "rust-analyzer",
  cmd = { "rust-analyzer" },
  root_dir = vim.fs.dirname(
    vim.fs.find({ "Cargo.toml", "rust-project.json" }, { upward = true })[1]
  ),
})
```

または `LspAttach` イベントでキーマップを設定:

```lua
vim.api.nvim_create_autocmd("LspAttach", {
  callback = function(ev)
    local buf = ev.buf
    vim.keymap.set('n', 'gd',         vim.lsp.buf.definition,   { buffer = buf })
    vim.keymap.set('n', 'gr',         vim.lsp.buf.references,   { buffer = buf })
    vim.keymap.set('n', 'K',          vim.lsp.buf.hover,        { buffer = buf })
    vim.keymap.set('n', '<leader>rn', vim.lsp.buf.rename,       { buffer = buf })
    vim.keymap.set('n', '<leader>ca', vim.lsp.buf.code_action,  { buffer = buf })
    vim.keymap.set('n', ']d',         vim.diagnostic.goto_next, { buffer = buf })
    vim.keymap.set('n', '[d',         vim.diagnostic.goto_prev, { buffer = buf })
    vim.keymap.set('n', '<leader>f',  function()
      vim.lsp.buf.format({ async = true })
    end, { buffer = buf })
  end,
})
```

### autocommand（`vim.api.nvim_create_autocmd`）

```lua
-- 保存時にフォーマット
vim.api.nvim_create_autocmd("BufWritePre", {
  pattern = "*.rs",
  callback = function()
    vim.lsp.buf.format({ async = false })
  end,
})
```

### デバッグ・確認コマンド

| コマンド | 動作 |
|----------|------|
| `:source %` | 現在の init.lua を再読み込み |
| `:checkhealth` | nvim の状態チェック |
| `:verbose map {key}` | キーマッピングの定義元を確認 |
| `:lua print(vim.inspect(vim.opt.tabstop:get()))` | オプション値の確認 |
| `:messages` | 直前のメッセージを確認 |

---

## ミッション一覧

| ミッション | 内容 | 難易度 |
|-----------|------|-------|
| [ミッション 1](mission-1/README.md) | `vim.opt` で基本オプションを設定する | ★☆☆ |
| [ミッション 2](mission-2/README.md) | `vim.keymap.set` で使いやすいキーマップを作る | ★★☆ |
| [ミッション 3](mission-3/README.md) | 組み込み LSP を `init.lua` で起動・設定する | ★★★ |
| [章末総合演習](summary/README.md) | 1〜8章の学習を活かした init.lua を自分で書き上げる | ★★★ |
