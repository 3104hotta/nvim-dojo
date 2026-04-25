vim.api.nvim_create_autocmd("LspAttach", {
  callback = function(ev)
    local buf = ev.buf
    local map = function(lhs, rhs, desc)
      vim.keymap.set('n', lhs, rhs, { buffer = buf, silent = true, desc = desc })
    end
    map('gd',         vim.lsp.buf.definition,                              'Go to definition')
    map('gr',         vim.lsp.buf.references,                              'Go to references')
    map('K',          vim.lsp.buf.hover,                                   'Hover doc')
    map('<leader>rn', vim.lsp.buf.rename,                                  'Rename symbol')
    map('<leader>ca', vim.lsp.buf.code_action,                             'Code action')
    map(']d',         vim.diagnostic.goto_next,                            'Next diagnostic')
    map('[d',         vim.diagnostic.goto_prev,                            'Prev diagnostic')
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
