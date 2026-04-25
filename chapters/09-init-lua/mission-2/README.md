# ミッション 2 — `vim.keymap.set` で使いやすいキーマップを作る

**難易度**: ★★☆  
**目安時間**: 10分

---

## 課題

`exercise/init.lua` に以下のキーマップを追加せよ。

1. `<Space>` を leader キーに設定する（`vim.g.mapleader = " "`）
2. ノーマルモードで `<Esc>` を押すと検索ハイライトが消えるようにする
3. `<leader>w` でファイルを保存する
4. `<leader>q` でウィンドウを閉じる（`:q<CR>`）
5. `<C-h/j/k/l>` でウィンドウ間を移動できるようにする（`Ctrl+w h` の代替）
6. ターミナルモードで `<C-n>` が `Ctrl+\ Ctrl+n`（ノーマルモードへ）の代わりになるようにする
7. `<leader>e` で netrw を開く（`:Ex<CR>`）

## 制約

- すべて `vim.keymap.set(mode, lhs, rhs, opts)` の形式で書く
- `{ noremap = true, silent = true }` オプションを付ける
- `desc` オプションで説明文を付ける

## ゴール

`goal/init.lua` と同一の内容になっており、設定したキーマップが動作すること。

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- `vim.g.mapleader` の設定は `keymap.set` より前に書く（順序が重要）
- ウィンドウ移動: `vim.keymap.set('n', '<C-h>', '<C-w>h', ...)`
- ターミナルモードは `'t'` で指定
- `':noh<CR>'` と `'<cmd>noh<CR>'` は等価（`<cmd>` 形式の方が推奨）

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```lua
vim.g.mapleader = " "

local map = function(mode, lhs, rhs, desc)
  vim.keymap.set(mode, lhs, rhs, { noremap = true, silent = true, desc = desc })
end

map('n', '<Esc>',    '<cmd>noh<CR>',      'Clear search highlight')
map('n', '<leader>w', '<cmd>w<CR>',       'Save file')
map('n', '<leader>q', '<cmd>q<CR>',       'Close window')
map('n', '<C-h>',    '<C-w>h',            'Move to left window')
map('n', '<C-j>',    '<C-w>j',            'Move to lower window')
map('n', '<C-k>',    '<C-w>k',            'Move to upper window')
map('n', '<C-l>',    '<C-w>l',            'Move to right window')
map('t', '<C-n>',    '<C-\\><C-n>',       'Exit terminal mode')
map('n', '<leader>e', '<cmd>Ex<CR>',      'Open netrw')
```

</details>
