-- hjkl-warn.lua — 同方向 hjkl を 3 回以上連続で押したら警告する
--
-- 使い方:
--   :luafile dojo/hjkl-warn.lua          （現在のセッションだけ有効化）
-- または init.lua に
--   dofile(vim.fn.stdpath('config') .. '/hjkl-warn.lua')
-- のように書いて常用化する。
--
-- 解除:
--   :lua require('dojo.hjkl-warn').disable()  （require 経由で読んだ場合）
-- または該当バッファだけ unmap:
--   :nunmap h | nunmap j | nunmap k | nunmap l
--
-- しきい値の変更:
--   :let g:dojo_hjkl_threshold = 5
-- （読み込み前に設定する。デフォルトは 3）

local M = {}

local THRESHOLD = vim.g.dojo_hjkl_threshold or 3
local last_dir, count = nil, 0
local hjkl_move = false

local MESSAGES = {
  h = 'h ×%d — leap, don\'t crawl: b · B · F{c} · 0 · ^',
  j = 'j ×%d — leap, don\'t crawl: } · <C-d> · G · {N}j · /pattern',
  k = 'k ×%d — leap, don\'t crawl: { · <C-u> · gg · {N}k · ?pattern',
  l = 'l ×%d — leap, don\'t crawl: w · W · e · f{c} · $',
}

-- 警告が表示済みなら cmdline をクリアする。
-- 表示は expr マッピング内で起きうるので vim.schedule で遅延実行。
local function clear_msg_if_warned()
  if count >= THRESHOLD then
    vim.schedule(function()
      vim.api.nvim_echo({ { '' } }, false, {})
    end)
  end
end

local function press(dir)
  return function()
    if last_dir == dir then
      count = count + 1
    else
      clear_msg_if_warned()  -- 方向が変わったら前の警告を消す
      last_dir, count = dir, 1
    end
    if count >= THRESHOLD then
      vim.notify(MESSAGES[dir]:format(count), vim.log.levels.WARN)
    end
    hjkl_move = true
    return dir
  end
end

local augroup = vim.api.nvim_create_augroup('DojoHjklWarn', { clear = true })

function M.enable()
  local opts = { noremap = true, expr = true, silent = false }
  vim.keymap.set('n', 'h', press('h'), opts)
  vim.keymap.set('n', 'j', press('j'), opts)
  vim.keymap.set('n', 'k', press('k'), opts)
  vim.keymap.set('n', 'l', press('l'), opts)

  -- 他の移動コマンドが使われたらカウンタをリセット。
  -- hjkl による CursorMoved はフラグで判別して無視する。
  vim.api.nvim_create_autocmd('CursorMoved', {
    group = augroup,
    callback = function()
      if hjkl_move then
        hjkl_move = false
      else
        clear_msg_if_warned()
        last_dir, count = nil, 0
      end
    end,
  })

  -- インサート / コマンドラインに入ったらリセット
  vim.api.nvim_create_autocmd({ 'InsertEnter', 'CmdlineEnter', 'BufLeave' }, {
    group = augroup,
    callback = function()
      clear_msg_if_warned()
      last_dir, count = nil, 0
    end,
  })
end

function M.disable()
  pcall(vim.keymap.del, 'n', 'h')
  pcall(vim.keymap.del, 'n', 'j')
  pcall(vim.keymap.del, 'n', 'k')
  pcall(vim.keymap.del, 'n', 'l')
  vim.api.nvim_clear_autocmds({ group = augroup })
  last_dir, count = nil, 0
end

-- :luafile で直接読み込んだときも有効化する
M.enable()

return M
