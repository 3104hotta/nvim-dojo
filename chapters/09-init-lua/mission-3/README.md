# ミッション 3 — 組み込み LSP を `init.lua` で起動・設定する

**難易度**: ★★★  
**目安時間**: 15分

---

## 課題

`exercise/init.lua` に rust-analyzer の LSP 設定を追加せよ。

1. `LspAttach` イベントで以下のキーマップを設定する:
   - `gd` → `vim.lsp.buf.definition`
   - `gr` → `vim.lsp.buf.references`
   - `K` → `vim.lsp.buf.hover`
   - `<leader>rn` → `vim.lsp.buf.rename`
   - `<leader>ca` → `vim.lsp.buf.code_action`
   - `]d` → `vim.diagnostic.goto_next`
   - `[d` → `vim.diagnostic.goto_prev`
   - `<leader>f` → `vim.lsp.buf.format({ async = true })`

2. `BufReadPre` イベントで rust-analyzer を起動する:
   ```lua
   vim.lsp.start({
     name = "rust-analyzer",
     cmd  = { "rust-analyzer" },
     root_dir = vim.fs.dirname(
       vim.fs.find({ "Cargo.toml" }, { upward = true })[1]
     ),
   })
   ```

3. 診断の表示をカスタマイズする:
   ```lua
   vim.diagnostic.config({
     virtual_text = true,
     signs        = true,
     underline    = true,
   })
   ```

## 制約

- `vim.lsp.config` (0.11+) ではなく `vim.lsp.start` (0.10互換) を使う
- `LspAttach` コールバック内でキーマップを設定する（バッファローカル）

## ゴール

設定した `init.lua` で Rust ファイルを開いたとき LSP が起動し、`gd` でジャンプできること。

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- `:checkhealth` で LSP の状態を確認できる
- `:LspInfo` で接続中のクライアントを確認
- `rust-analyzer` が見つからない場合: `which rust-analyzer` でパスを確認
- `autocmd` の `LspAttach` は LSP がバッファにアタッチされるたびに発火する

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```lua
vim.api.nvim_create_autocmd("LspAttach", {
  callback = function(ev)
    local buf = ev.buf
    local map = function(lhs, rhs, desc)
      vim.keymap.set('n', lhs, rhs, { buffer = buf, silent = true, desc = desc })
    end
    map('gd',         vim.lsp.buf.definition,                    'Go to definition')
    map('gr',         vim.lsp.buf.references,                    'Go to references')
    map('K',          vim.lsp.buf.hover,                         'Hover doc')
    map('<leader>rn', vim.lsp.buf.rename,                        'Rename symbol')
    map('<leader>ca', vim.lsp.buf.code_action,                   'Code action')
    map(']d',         vim.diagnostic.goto_next,                  'Next diagnostic')
    map('[d',         vim.diagnostic.goto_prev,                  'Prev diagnostic')
    map('<leader>f',  function() vim.lsp.buf.format({ async = true }) end, 'Format')
  end,
})

vim.api.nvim_create_autocmd("BufReadPre", {
  pattern = "*.rs",
  callback = function()
    vim.lsp.start({
      name     = "rust-analyzer",
      cmd      = { "rust-analyzer" },
      root_dir = vim.fs.dirname(
        vim.fs.find({ "Cargo.toml" }, { upward = true })[1]
      ),
    })
  end,
})

vim.diagnostic.config({
  virtual_text = true,
  signs        = true,
  underline    = true,
})
```

</details>
