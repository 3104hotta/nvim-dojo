vim.g.mapleader = " "

local map = function(mode, lhs, rhs, desc)
  vim.keymap.set(mode, lhs, rhs, { noremap = true, silent = true, desc = desc })
end

map('n', '<Esc>',     '<cmd>noh<CR>',  'Clear search highlight')
map('n', '<leader>w', '<cmd>w<CR>',    'Save file')
map('n', '<leader>q', '<cmd>q<CR>',    'Close window')

map('n', '<C-h>', '<C-w>h', 'Move to left window')
map('n', '<C-j>', '<C-w>j', 'Move to lower window')
map('n', '<C-k>', '<C-w>k', 'Move to upper window')
map('n', '<C-l>', '<C-w>l', 'Move to right window')

map('t', '<C-n>', '<C-\\><C-n>', 'Exit terminal mode')

map('n', '<leader>e', '<cmd>Ex<CR>', 'Open netrw')
