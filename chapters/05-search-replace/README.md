# 第5章 検索・置換

## 概要

「あの変数どこだっけ」を一瞬で解決し、複数ファイルにまたがる一括置換も nvim 単体でこなす。
grep や sed に頼らず、エディタの中で完結させる技術を身につける。

---

## コマンド早見表

### 検索

| コマンド | 由来 | 動作 |
|----------|------|------|
| `/{pattern}` | スラッシュ＝前方検索 | 前方検索 |
| `?{pattern}` | クエスチョン＝後方検索 | 後方検索 |
| `n` | **n**ext | 次のマッチへ |
| `N` | **N**ext（逆方向） | 前のマッチへ |
| `*` | 単語ハイライト | カーソル下の単語を前方検索 |
| `#` | `*` の逆方向 | カーソル下の単語を後方検索 |
| `g*` | **g**lobal + `*` | 部分一致で前方検索（単語境界なし） |
| `:noh` | **no h**ighlight | 検索ハイライトを消す |

**パターンのコツ**:
- `\<word\>` — 単語境界つきの完全一致
- `\v` — **v**ery magic モード（正規表現が書きやすくなる）
- `\c` / `\C` — **c**ase insensitive / **C**ase sensitive

### 置換（`:s`）

| コマンド | 由来 | 動作 |
|----------|------|------|
| `:s/old/new/` | **s**ubstitute | 現在行の最初のマッチを置換 |
| `:s/old/new/g` | **g**lobal（行内すべて） | 現在行の全マッチを置換 |
| `:%s/old/new/g` | `%` = ファイル全体 | ファイル全体を置換 |
| `:%s/old/new/gc` | **c**onfirm（確認しながら） | 全体を確認しながら置換（`y/n/a/q`） |
| `:'<,'>s/old/new/g` | ビジュアル選択範囲 | 選択範囲内を置換 |
| `:%s/\<old\>/new/g` | `\<\>` = 単語境界 | 単語境界つきで置換 |
| `:%s/old/new/gI` | **I** = case sensitive を強制 | 大文字小文字を区別して置換 |

**置換時の参照**:
- `&` — マッチした文字列全体
- `\1`, `\2` — キャプチャグループ
- 例: `:%s/\(foo\)_\(bar\)/\2_\1/g` — foo_bar → bar_foo

### vimgrep（複数ファイル検索）

| コマンド | 由来 | 動作 |
|----------|------|------|
| `:vimgrep /pattern/ **/*.rs` | vim 内蔵の **grep** | Rust ファイル全体を検索 |
| `:vimgrep /pattern/ %` | `%` = 現在ファイル | 現在のファイルを検索 |
| `:copen` | quickfix **open** | quickfix ウィンドウを開く |
| `:cclose` | quickfix **close** | quickfix ウィンドウを閉じる |
| `:cn` | quickfix **n**ext | 次のマッチへ |
| `:cp` | quickfix **p**revious | 前のマッチへ |
| `:cfirst` | **first** entry | 最初のマッチへ |
| `:clast` | **last** entry | 最後のマッチへ |
| `:cdo s/old/new/g` | quickfix **do**（各エントリ実行） | quickfix リストの全箇所で置換 |

### quickfix の活用

| コマンド | 由来 | 動作 |
|----------|------|------|
| `:clist` | quickfix **list** | quickfix リストを表示 |
| `:cc {N}` | **c**urrent **c**hange | N 番目のエントリへジャンプ |
| `Ctrl+w Enter` | window + 開く | quickfix の項目を新ウィンドウで開く |

---

## ミッション一覧

| ミッション | 内容 | 難易度 |
|-----------|------|-------|
| [ミッション 1](mission-1/README.md) | `/` 検索と `n/N`・`*/#` で素早くナビゲート | ★☆☆ |
| [ミッション 2](mission-2/README.md) | `:%s` で正規表現を使った置換 | ★★☆ |
| [ミッション 3](mission-3/README.md) | vimgrep + quickfix + `:cdo` で複数ファイル一括置換 | ★★★ |
| [章末総合演習](summary/README.md) | 検索・置換・quickfix を組み合わせた大規模変更 | ★★★ |
