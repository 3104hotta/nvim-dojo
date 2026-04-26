# 第8章 git 操作

## 概要

ターミナルとエディタを行き来せず、nvim の中で git 操作を完結させる。
`:terminal`・`:!` による git コマンド実行と、`vimdiff` によるコンフリクト解消を習得する。

---

## コマンド早見表

### `:terminal`（組み込みターミナル）

| コマンド | 由来 | 動作 |
|----------|------|------|
| `:terminal` | **terminal** | カレントウィンドウでターミナルを起動 |
| `:sp \| terminal` | split + terminal | 水平分割でターミナルを起動 |
| `:vsp \| terminal` | vsplit + terminal | 垂直分割でターミナルを起動 |
| `i` / `a` | **i**nsert / **a**ppend | ターミナルモードに入る |
| `Ctrl+\ Ctrl+n` | `\` で抜ける + **n**ormal モード | ノーマルモードへ |

### `:!`（シェルコマンドの即時実行）

| コマンド | 由来 | 動作 |
|----------|------|------|
| `:!{cmd}` | `!` ＝ シェル実行 | コマンドを実行して出力を表示 |
| `:!git status` | 同上 | git status を実行 |
| `:!git add %` | `%` ＝ 現在ファイル名 | 現在のファイルをステージング |
| `:!git diff` | 同上 | diff を表示 |
| `:r !{cmd}` | **r**ead from command | コマンド出力をカーソル位置に挿入 |

### vimdiff

| コマンド | 由来 | 動作 |
|----------|------|------|
| `vimdiff file1 file2` | vim + diff | ターミナルから2ファイルを diff で開く |
| `:diffthis` | diff + this | 現在のウィンドウを diff 対象にする |
| `:diffoff` | diff + off | diff モードを終了 |
| `:diffupdate` | diff + update | diff を再計算 |
| `]c` | 次の **c**hange | 次の diff チャンクへ |
| `[c` | 前の **c**hange | 前の diff チャンクへ |
| `do` | **d**iff **o**btain | 相手ウィンドウから変更を取得 |
| `dp` | **d**iff **p**ut | 相手ウィンドウへ変更を送る |

### マージコンフリクト解消

コンフリクトマーカーを手動で扱う場合:

| コマンド | 由来 | 動作 |
|----------|------|------|
| `/<<<` | 検索 | コンフリクトマーカーを検索 |
| `dd` | **d**elete line | マーカー行を削除 |
| `[c` / `]c` | 前後の change | vimdiff でのチャンク移動 |

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
