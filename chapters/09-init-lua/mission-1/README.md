# ミッション 1 — `vim.opt` で基本オプションを設定する

**難易度**: ★☆☆  
**目安時間**: 5〜10分

---

## 課題

`exercise/init.lua` を開き、以下のオプションを設定せよ。

1. 行番号を有効化し、相対行番号も有効化する（`number` と `relativenumber`）
2. Tab をスペース4個に設定する（`expandtab`・`tabstop`・`shiftwidth`）
3. 検索をインクリメンタルにして、大文字を含む場合だけ大文字小文字を区別する（`incsearch`・`ignorecase`・`smartcase`）
4. カーソル行をハイライトし、折り返しを無効化する（`cursorline`・`wrap`）
5. ファイルをスワップなしで開く（`swapfile = false`）

## 制約

- すべて `vim.opt.{name} = value` の形式で書く
- 変更後に `:source %` で即座に反映させて確認する

## ゴール

`goal/init.lua` と同一の内容になっていること。
nvim を再起動すると設定が反映されること。

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- `:set number?` で現在の値を確認できる
- `:lua print(vim.opt.tabstop:get())` で lua 側から値を確認
- `vim.opt` は各オプションのオブジェクト。`:set` の lua 版
- boolean オプション: `vim.opt.number = true`
- 数値オプション: `vim.opt.tabstop = 4`

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```lua
vim.opt.number         = true
vim.opt.relativenumber = true

vim.opt.expandtab  = true
vim.opt.tabstop    = 4
vim.opt.shiftwidth = 4

vim.opt.incsearch  = true
vim.opt.ignorecase = true
vim.opt.smartcase  = true

vim.opt.cursorline = true
vim.opt.wrap       = false

vim.opt.swapfile   = false
```

</details>
