# 第8章 git 操作

## 概要

ターミナルとエディタを行き来せず、nvim の中で git 操作を完結させる。
`:terminal`・`:!` による git コマンド実行と、`vimdiff` によるコンフリクト解消を習得する。

---

## コマンド早見表

### `:terminal`（組み込みターミナル）

| コマンド | 動作 |
|----------|------|
| `:terminal` | カレントウィンドウでターミナルを起動 |
| `:sp \| terminal` | 水平分割でターミナルを起動 |
| `:vsp \| terminal` | 垂直分割でターミナルを起動 |
| `i` / `a` | ターミナルモードに入る |
| `Ctrl+\ Ctrl+n` | ターミナルモードを抜けてノーマルモードへ |

### `:!`（シェルコマンドの即時実行）

| コマンド | 動作 |
|----------|------|
| `:!git status` | git status を実行して出力を表示 |
| `:!git add %` | 現在のファイルをステージング |
| `:!git diff` | diff を表示 |
| `:!git log --oneline -10` | 直近10コミットをログ表示 |
| `:r !git log --oneline -5` | コマンド出力をカーソル位置に挿入 |

### vimdiff

| コマンド | 動作 |
|----------|------|
| `vimdiff file1 file2` | ターミナルから2ファイルを diff で開く |
| `:diffthis` | 現在のウィンドウを diff 対象にする |
| `:diffoff` | diff モードを終了 |
| `:diffupdate` | diff を再計算 |
| `]c` | 次の diff チャンクへ |
| `[c` | 前の diff チャンクへ |
| `do` | 相手ウィンドウの変更を取得（diff obtain） |
| `dp` | 相手ウィンドウへ変更を送る（diff put） |

### マージコンフリクト解消（vimdiff 3-way）

```
:e conflicted_file.rs
:Gdiff              ← fugitive がない場合は vimdiff を手動で使う
```

コンフリクトマーカーを手動で探す場合:

| コマンド | 動作 |
|----------|------|
| `/<<<` | コンフリクトマーカーを検索 |
| `dd` | マーカー行を削除 |
| `[c` / `]c` | vimdiff でのチャンク移動 |

### 実用的な git ワークフロー（nvim 内完結）

```
1. :vsp | terminal        ← 右ペインにターミナル
2. (ターミナルで) git log --oneline -5
3. (エディタで) ファイル編集・保存
4. :!git diff %           ← 変更確認
5. :!git add %
6. :!git commit -m "..."
```

---

## ミッション一覧

| ミッション | 内容 | 難易度 |
|-----------|------|-------|
| [ミッション 1](mission-1/README.md) | `:terminal` と `:!` で git 操作を nvim 内で完結させる | ★☆☆ |
| [ミッション 2](mission-2/README.md) | vimdiff でファイルの差分を確認・マージ | ★★☆ |
| [ミッション 3](mission-3/README.md) | マージコンフリクトを vimdiff で解消する | ★★★ |
| [章末総合演習](summary/README.md) | 修正・確認・コミットのサイクルを nvim 内で完走 | ★★★ |
