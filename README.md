# Neovim 実践学習

プラグインなしの標準 Neovim を、実践課題を通じて体系的に学ぶ演習集。

## 使い方

1. 興味のある章・ミッションから始める（順番通りでなくてもよい）
2. `chapters/XX-*/README.md` でコマンドを確認する
3. `mission-N/README.md` の課題を読み、`exercise.*` を nvim で開いて取り組む
4. 完了したら `progress.md` のチェックボックスを更新する

```bash
./progress.sh   # 進捗サマリーを表示
```

## 章構成

| 章 | タイトル | 学習内容 |
|----|----------|----------|
| [01](chapters/01-movement/) | 移動の高速化 | f/t/w/b/gg/G/Ctrl+d |
| [02](chapters/02-modes/) | モードとインサート | i/a/o/O/I/A/s/S |
| [03](chapters/03-verb-noun/) | 動詞＋名詞 | operator + text object |
| [04](chapters/04-repeat/) | 繰り返しの3種 | `.`・マクロ・`{N}` |
| [05](chapters/05-search-replace/) | 検索・置換 | `/`・`:%s`・vimgrep・quickfix |
| [06](chapters/06-files-buffers/) | ファイル・バッファ・ウィンドウ | netrw・`:ls`・分割・タブ・サイズ変更 |
| [07](chapters/07-lsp/) | LSP | gd/gr/K・診断・フォーマット |
| [08](chapters/08-git/) | git 操作 | `:terminal`・`:!git`・vimdiff |
| [09](chapters/09-init-lua/) | init.lua | 基本設定・キーマッピング・LSP 起動 |

## 前提

- vim の基本操作（hjkl・モード切り替え）は習得済み
- nvim 0.10 以上
- 第7章は `rust-analyzer` のインストールが必要（章内で案内）
- macOS / Linux
