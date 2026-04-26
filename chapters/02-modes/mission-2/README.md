# ミッション 2 — s/S/c/C で範囲を書き直す

**難易度**: ★★☆
**目安時間**: 5〜10分

---

## 課題

`exercise.rs` のコードを、`s`・`S`・`c{motion}`・`C` を使って書き直せ。
「削除してインサートモードへ」という動作を意識しながら最小手数で変更する。

1. `let result = vec![];` の行を **行ごと** `let result = Vec::new();` に書き直す
2. `Display` 実装の `"invalid input: {}"` を `"bad input: {}"` に変更する
3. `let x = calculate();` の `calculate();` の部分を **カーソルから行末まで** `calculate() * 2;` に変更する
4. `println!("done");` の行を **空行** に変更する（行の内容を消してインサートモードに入り、即 Esc）
5. `let factor = 7u64;` の `7` を `9` に変更する（**1文字だけ** 変える）

## 制約

- 課題1: `S` で行全体を置き換える
- 課題2: `ci"` でクォート内を変更
- 課題3: `C`（= `c$`）でカーソルから行末まで変更
- 課題4: `S` → `<Esc>` で行を空にする
- 課題5: `s` で1文字だけ置き換える

## ゴール

`goal.rs` と同一の状態にすること。

```
1. let result = vec![];
   → let result = Vec::new();

2. "invalid input: {}"  →  "bad input: {}"

3. let x = calculate();
   → let x = calculate() * 2;

4. println!("done");
   → （空行）

5. let factor = 7u64;
   → let factor = 9u64;
```

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- `S` は現在行を削除してインサートモードに入る（インデントは自動）
- `ci"` = change inside double-quotes
- `C` = `c$` と同じ（カーソルから行末を変更）
- `s` は1文字だけ変えたいときに使う（`cl` と同じだが1キー短い）
- `f7` で行内の `7` の位置にジャンプ → `s9<Esc>` で1文字書き換え

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```
1. /let result → S → let result = Vec::new();<Esc>
2. /invalid    → ci" → bad input: {}<Esc>
3. /let x      → fc → C → calculate() * 2;<Esc>
4. /println    → S → <Esc>
5. /factor     → f7 → s → 9<Esc>
```

</details>

---

## 別解

<details>
<summary>別解を見る（上達したら試してみよう）</summary>

- 課題3は `f;` でセミコロンへ → `b` で戻る → `cf;` で `;` まで変更も可
- 課題5は `f7` → `r9` で1文字置換（モードを切り替えずに済む）
- `cit` (change inside tag) はHTMLやXMLで有効

</details>

---

## 演習リセット

演習で `exercise.rs` を編集した後、コミット前に元の状態へ戻すこと。
そうしないと次回の演習で「最初から `goal.rs` と同じ」になってしまう。

```bash
# このミッションだけリセット
scripts/reset.sh 02-modes/mission-2

# 全ミッションをまとめてリセット
scripts/reset.sh
```
