# 章末総合演習 — 修正・確認・コミットを nvim 内で完走

**難易度**: ★★★  
**目安時間**: 20〜25分

---

## シナリオ

バグ報告を受けた。`summary/project/` の Rust コードに修正を加え、
git での管理作業まで nvim を閉じずに完了させよ。

---

## 課題

### フェーズ 1 — 状況把握

1. `:!git log --oneline -10` で最近の変更を確認する
2. `:!git diff HEAD~1 HEAD -- src/` で直前のコミットの変更を確認する
3. `:vsp | terminal` でターミナルを開き `git blame src/buggy.rs` でバグの混入コミットを特定する

### フェーズ 2 — 修正

4. `src/buggy.rs` のバグを修正する（LSP の診断を参考にする）
5. `:!git diff %` で変更内容を確認する
6. `vimdiff src/buggy.rs src/expected.rs` で期待値と比較する（`:e src/buggy.rs | vsp src/expected.rs | windo diffthis`）
7. 差分がなくなるまで修正する

### フェーズ 3 — コミット

8. `:!git add src/buggy.rs`
9. ターミナルで `git commit -m "fix: correct validation logic in buggy.rs"`
10. `:!git log --oneline -3` で最終確認する

## ゴール

`git log --oneline -1` に修正コミットが表示されること。

---

## 振り返りチェックリスト

- `:!{cmd}` と `:terminal` の使い分けができたか？
- vimdiff で `do`/`dp` を意図通りに使えたか？
- nvim を閉じずにすべての操作が完結したか？
