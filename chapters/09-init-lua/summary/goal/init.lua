-- ============================================================
--  Reference init.lua — built up from chapters 1–8
-- ============================================================

vim.g.mapleader      = " "
vim.g.maplocalleader = "\\"

-- ---- Options --------------------------------------------------
local opt = vim.opt

opt.number         = true
opt.relativenumber = true

opt.expandtab  = true
opt.tabstop    = 4
opt.shiftwidth = 4
opt.smartindent = true

opt.wrap       = false
opt.cursorline = true
opt.signcolumn = "yes"

opt.incsearch  = true
opt.ignorecase = true
opt.smartcase  = true
opt.hlsearch   = true

opt.splitright = true
opt.splitbelow = true

opt.swapfile   = false
opt.updatetime = 250
opt.termguicolors = true

vim.cmd.colorscheme("habamax")

-- ---- Keymaps --------------------------------------------------
local map = function(mode, lhs, rhs, desc)
  vim.keymap.set(mode, lhs, rhs, { noremap = true, silent = true, desc = desc })
end

map('n', '<Esc>',      '<cmd>noh<CR>',   'Clear search highlight')
map('n', '<leader>w',  '<cmd>w<CR>',     'Save file')
map('n', '<leader>q',  '<cmd>q<CR>',     'Close window')
map('n', '<leader>e',  '<cmd>Ex<CR>',    'Open netrw')

map('n', '<C-h>', '<C-w>h', 'Move to left window')
map('n', '<C-j>', '<C-w>j', 'Move to lower window')
map('n', '<C-k>', '<C-w>k', 'Move to upper window')
map('n', '<C-l>', '<C-w>l', 'Move to right window')

map('n', '<leader>cn', '<cmd>cnext<CR>', 'Next quickfix')
map('n', '<leader>cp', '<cmd>cprev<CR>', 'Prev quickfix')

map('t', '<C-n>', '<C-\\><C-n>', 'Exit terminal mode')

-- ---- LSP ------------------------------------------------------
vim.api.nvim_create_autocmd("LspAttach", {
  callback = function(ev)
    local buf = ev.buf
    local m = function(lhs, rhs, desc)
      vim.keymap.set('n', lhs, rhs, { buffer = buf, silent = true, desc = desc })
    end
    m('gd',         vim.lsp.buf.definition,                              'Go to definition')
    m('gr',         vim.lsp.buf.references,                              'Go to references')
    m('K',          vim.lsp.buf.hover,                                   'Hover doc')
    m('<leader>rn', vim.lsp.buf.rename,                                  'Rename symbol')
    m('<leader>ca', vim.lsp.buf.code_action,                             'Code action')
    m(']d',         vim.diagnostic.goto_next,                            'Next diagnostic')
    m('[d',         vim.diagnostic.goto_prev,                            'Prev diagnostic')
    m('<leader>f',  function() vim.lsp.buf.format({ async = true }) end, 'Format')
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

-- ---- Autocmds -------------------------------------------------
vim.api.nvim_create_autocmd("BufWritePre", {
  pattern  = "*.rs",
  callback = function() vim.lsp.buf.format({ async = false }) end,
})

vim.api.nvim_create_autocmd("TermOpen", {
  callback = function()
    vim.opt_local.number         = false
    vim.opt_local.relativenumber = false
    vim.opt_local.signcolumn     = "no"
  end,
})
