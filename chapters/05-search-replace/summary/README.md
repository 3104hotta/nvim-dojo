# 章末総合演習 — 検索・置換・quickfix を組み合わせた大規模変更

**難易度**: ★★★  
**目安時間**: 20〜30分

---

## シナリオ

古い Rust クレートを新しいバージョンにアップデートする移行作業。
`src/` 以下の複数ファイルに対して、nvim 単体で変更を完了させよ。

## 課題

### フェーズ 1 — 現状把握（検索）

1. `use old_crate::` を含む行を全ファイルから検索して quickfix に出力する
2. `old_crate` の参照が何箇所あるかカウントする（`:vimgrep` 後に `:clist` で確認）
3. `deprecated` コメントがついている関数を探す（`/\/\/ deprecated` または `vimgrep`）

### フェーズ 2 — 一括置換

4. `old_crate::Error` → `new_crate::AppError` に全ファイルで置換する
5. `old_crate::Result` → `new_crate::AppResult` に全ファイルで置換する
6. `use old_crate` → `use new_crate` に変更する（`use` 行のみ対象）
7. バージョンコメント `// v1.x` を `// v2.0` に変更する

### フェーズ 3 — 確認・クリーンアップ

8. 変更後に再度 `:vimgrep /old_crate/ **/*.rs` して残りがないことを確認する
9. 各ファイルで trailing whitespace を削除する（`:cfdo %s/\s\+$//g | update`）
10. quickfix ウィンドウで全ファイルを順に開き、変更が正しいことを目視確認する

## ゴール

`src/` 以下の全ファイルで `old_crate` への参照がゼロになっていること。

---

## 振り返りチェックリスト

- `:cdo` と `:cfdo` の違いを説明できるか？
- `\<word\>` の単語境界指定が必要な場面を挙げられるか？
- `c` フラグ（確認置換）をいつ使うべきか？
- `vimgrep` と外部の `grep` を使い分ける基準は？
