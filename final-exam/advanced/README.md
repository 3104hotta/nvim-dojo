# 最終総合演習 — 上級編

**対象**: 全章を習得し、実務で nvim を使っている人
**目安時間**: 1問あたり 10〜15分（全12問 ≒ 150分）

---

## 構成

3つのテーマ × 4小問の構成。各小問は独立して取り組める。

| テーマ | 内容 | 題材 |
|--------|------|------|
| A | OSS コードリファクタ | `exercise-1/` |
| B | v1 → v2 バージョン移行 | `exercise-2/` |
| C | HTTP サーバー実装タイムアタック | `exercise-3/` |

---

## テーマ A — OSS コードリファクタ

`exercise-1/` のコードを段階的にリファクタする。各問は前の問の結果を引き継いで進める。

> ⚠️ **A2〜A4 は前の小問の完了状態を前提とする**。途中から始めたい場合や再挑戦時は、
> リポジトリルートで `./final-exam/advanced/reset.sh` を実行して題材を初期状態に戻し、
> A1 から順に進めること。

### 問題 A1 — Error 型を `AppError` にリネーム（10分）

`Error` enum を `AppError` にリネームし、全参照箇所に伝播させる。

- LSP の `<leader>rn`（rename）を使うこと
- `:%s` 禁止
- 完了条件: `cargo check` がエラーなしで通る

| 回数 | タイム |
|------|--------|
| 1回目 | |
| 自己ベスト | |

### 問題 A2 — `?` 演算子と `From` impl で統一（15分）

`async fn fetch` 内の `.map_err(Error::Io)?` パターンを、`?` 演算子と `From<io::Error>` 実装に置き換える。

- `From` impl はすでに存在することを確認する
- `.map_err(...)` を削除して `?` だけにする
- 同様のパターンが他に何箇所あるかを `vimgrep` で確認する

| 回数 | タイム |
|------|--------|
| 1回目 | |
| 自己ベスト | |

### 問題 A3 — `#[allow(dead_code)]` を整理（10分）

`#[allow(dead_code)]` がついているすべての関数を確認し、用途を判断して **削除または実装** する。

- `:vimgrep /allow(dead_code)/ **/*.rs` で全箇所を quickfix に出す
- quickfix を順に辿り、`d` キー（マクロでもよい）で削除
- 完了条件: `#[allow(dead_code)]` が0件になる

| 回数 | タイム |
|------|--------|
| 1回目 | |
| 自己ベスト | |

### 問題 A4 — `impl` ブロック内のメソッドを並び替え（15分）

`impl Store` の中身を、以下の順序で並べ替える:

1. パブリック API（`new`、`add` など）
2. パブリック非同期メソッド（`fetch` など）
3. プライベートメソッド（`key_for` など）

- メソッド単位の移動は `V}` でブロック選択 → `d` → ペースト先で `P`
- 並び替え後 `cargo fmt` で整形して `cargo clippy` を通す

| 回数 | タイム |
|------|--------|
| 1回目 | |
| 自己ベスト | |

---

## テーマ B — v1 → v2 バージョン移行

`exercise-2/v1/` の実装を `v2/` のスタイルに段階的に移行する。

> ⚠️ **B2〜B4 は前の小問の完了状態を前提とする**（`v1/src/lib.rs` を段階的に v2 化するため）。
> 途中から始めたい場合や再挑戦時は `./final-exam/advanced/reset.sh` で初期状態に戻し、
> B1 から順に進めること。

### 問題 B1 — vimdiff で v1/v2 の差分を把握（10分）

```bash
vimdiff exercise-2/v1/src/lib.rs exercise-2/v2/src/lib.rs
```

- `]c` / `[c` で差分チャンクを順に確認する
- 何箇所違うかをカウントし、紙に記録する
- どこを v2 スタイルに揃えるべきか方針を立てる
- **このフェーズではまだ編集しない**

| 回数 | タイム |
|------|--------|
| 1回目 | |
| 自己ベスト | |

### 問題 B2 — 手動 `Clone`/`PartialEq`/`Debug` を derive に置き換え（10分）

`v1/src/lib.rs` の `Record` の手動 trait 実装3つ（`Clone`・`PartialEq`・`Debug`）を削除し、`#[derive(Debug, Clone, PartialEq, Eq)]` に置き換える。

- 削除には `daB`（波括弧ブロックを削除）または `Vap d`（段落削除）を使う
- 完了条件: コードが30行以上短くなる

| 回数 | タイム |
|------|--------|
| 1回目 | |
| 自己ベスト | |

### 問題 B3 — `panic!` を `Result` に置き換え（15分）

`v1` の `parse` 関数の `panic!` を、v2 と同様に `Result<Record, AppError>` を返す形に修正する。

- v2 の `parse` を参考にする（vimdiff で見比べる）
- `AppError::Empty` と `AppError::Invalid(String)` を使う
- `AppError::Parse` バリアントが v1 にない場合は追加する
- `From<std::num::ParseIntError>` 実装も追加する

| 回数 | タイム |
|------|--------|
| 1回目 | |
| 自己ベスト | |

### 問題 B4 — v2 にしかない `parse_with_tags` を v1 に移植（15分）

v2 の `parse_with_tags` を v1 にコピーし、v1 のスタイル（v2 にもある形）で動くように調整する。

- v2 ペインで `:e v2/src/lib.rs` → 該当関数を `Vap y`
- v1 ペインで `p`
- `cargo build` で v1 がビルドできることを確認する

| 回数 | タイム |
|------|--------|
| 1回目 | |
| 自己ベスト | |

---

## テーマ C — HTTP サーバー実装タイムアタック

`exercise-3/` の未実装スケルトンを段階的に埋めていく。LSP の補完・診断を活用しながら、各小問を時間内に完了させる。

### 共通の縛り

- `hjkl` によるカーソル移動禁止
- ファイルを開くのは netrw（`<leader>e`）のみ、`:e` 禁止
- LSP の補完・コードアクション・診断はフル活用してよい

### 問題 C1 — `Config` を実装（10分）

`src/config.rs` で以下を実装する:

- `Config::new(host: &str, port: u16) -> Self`
- `impl fmt::Display for Config`（`<host>:<port>` 形式）

完了条件: `cargo check -p final-advanced-3 --no-default-features` が `config.rs` 関連のエラーを出さない。

| 回数 | タイム |
|------|--------|
| 1回目 | |
| 自己ベスト | |

### 問題 C2 — `Handler` trait を `EchoHandler` に実装（10分）

`src/handler.rs` で:

- `impl Handler for EchoHandler`
- `handle_get` は `Response::ok(path)` を返す
- `handle_post` は `Response::ok(&format!("{}:{}", path, body))` を返す

| 回数 | タイム |
|------|--------|
| 1回目 | |
| 自己ベスト | |

### 問題 C3 — `Router` の `add_route` と `dispatch` を実装（15分）

`src/router.rs` で:

- `add_route(&mut self, method: &str, path: &str)` → `routes` に `"{method} {path}"` をキーとして登録
- `dispatch(&self, method: &str, path: &str, body: Option<&str>) -> Response`
  - 未登録ルートは `Response { status: 404, body: "Not Found".to_string() }`
  - GET は `handler.handle_get(path)`、POST は `handler.handle_post(path, body.unwrap_or(""))`

| 回数 | タイム |
|------|--------|
| 1回目 | |
| 自己ベスト | |

### 問題 C4 — `main()` を仕上げて `cargo build` を通す（10分）

`src/main.rs` の `todo!()` を埋める:

- `Config::new("127.0.0.1", 8080)` を作って `Display` で表示
- `EchoHandler` を持つ `Router` を作る
- `add_route("GET", "/")` などを登録
- `dispatch("GET", "/", None)` を呼んで結果を表示

完了条件: `cargo build` がエラーなしで通る。

| 回数 | タイム |
|------|--------|
| 1回目 | |
| 自己ベスト | |

---

## 全12問の進捗チェックリスト

このセクションのチェックを、自分で `[ ]` → `[x]` に書き換えて使う。

```
テーマ A — リファクタ
  [ ] A1 Error → AppError リネーム
  [ ] A2 ? + From で統一
  [ ] A3 dead_code 整理
  [ ] A4 impl ブロック並び替え

テーマ B — バージョン移行
  [ ] B1 vimdiff で差分把握
  [ ] B2 手動 trait → derive
  [ ] B3 panic → Result
  [ ] B4 parse_with_tags 移植

テーマ C — タイムアタック実装
  [ ] C1 Config
  [ ] C2 Handler
  [ ] C3 Router
  [ ] C4 main + cargo build
```

12問すべてクリアしたら `progress.md` の「上級編」をチェック。
