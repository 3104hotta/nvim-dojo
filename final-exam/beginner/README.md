# 最終総合演習 — 初級編

**対象**: 各章のミッション 1〜2 レベルをクリアできた人  
**目安時間**: 1問あたり 5〜10分

---

## 問題 1 — ワンファイルリファクタ（移動・編集）

`exercise-1/src/main.rs` を開き、以下をすべて完了させよ。

- `hjkl` を使わずにすべての変更を行うこと
- `f/t/w/b/gg/G/Ctrl+d` を積極的に使うこと

### 変更内容

1. `fn main` の最初の行に `println!("=== App Start ===");` を追加する
2. `let config = Config::default();` を `let config = Config::load()?;` に変更する
3. `Result<()>` を戻り値に追加する（`fn main()` → `fn main() -> Result<(), Box<dyn std::error::Error>>`）
4. ファイル末尾に `Ok(())` を追加する
5. 未使用のインポート行（`use std::collections::HashMap;`）を削除する

### 記録

| 回数 | タイム |
|------|--------|
| 1回目 | |
| 2回目 | |
| 自己ベスト | |

---

## 問題 2 — テキストオブジェクト特訓

`exercise-2/src/lib.rs` を開き、以下の変換を operator + text object のみで行え。

1. すべての `String::from("...")` → `"...".to_string()` に変換する（5箇所）
2. `unwrap()` → `?` に変換する（4箇所）
3. `vec![...]` の内容をすべてヤンクして別の `Vec` に貼り付ける

### 記録

| 回数 | タイム |
|------|--------|
| 1回目 | |
| 2回目 | |
| 自己ベスト | |

---

## 問題 3 — 検索・置換ミックス

`exercise-3/src/config.rs` で以下の変換を行え。

1. `timeout_ms` をすべて `timeout_millis` にリネームする（単語境界を使う）
2. trailing whitespace を削除する
3. `debug` ログを `trace` に変更する（4箇所、`n.` で繰り返す）

### 記録

| 回数 | タイム |
|------|--------|
| 1回目 | |
| 2回目 | |
| 自己ベスト | |
