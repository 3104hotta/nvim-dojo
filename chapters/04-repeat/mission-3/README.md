# ミッション 3 — `{N}` プレフィクスとマクロを組み合わせた高速編集

**難易度**: ★★★  
**目安時間**: 10〜15分

---

## 課題

`exercise.rs` に対して、3種類の繰り返し機構を使い分けながら編集せよ。

**セクション A — 数値プレフィクス**:
1. `fn process` の関数本体を4回複製して `fn process_1` 〜 `fn process_4` を作る（`yap` + `4p`）
2. カーソルを20行下へ移動する（`20j`）
3. インデントを3レベル増やす（`3>>`）

**セクション B — マクロ + 数値プレフィクス**:
4. 以下のパターンが8行ある。マクロで1行変換を記録し `7@a` で残りに適用せよ
   ```
   before: log::debug!("msg_{}", i);
   after:  tracing::debug!(msg = "msg_{}", index = i);
   ```

**セクション C — ドット繰り返しの設計**:
5. `use crate::error::Error;` を `use crate::error::{Error, Result};` に変更する変更を1回行い、同じパターンの行（3行）に `.` を使って適用する

## 制約

- セクション A はすべて数値プレフィクスを使うこと
- セクション B はマクロを使うこと（`:%s` 禁止）
- セクション C は `.` を使うこと

## ゴール

`goal.rs` と同一の状態にすること。

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- `yap` で段落（関数ブロック）をヤンク → `4p` で4回ペースト
- マクロ内で `f(` や `ct,` などの motion を使うと柔軟に対応できる
- 課題5は `ci{` で `{Error}` 内を変更する設計にすると `.` で繰り返しやすい

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```
A-1: /fn process → yap → 4p
     → 各関数名を手動で process_1 〜 4 に変更（または別マクロ）

A-2: 20j

A-3: 3>>

B-4: /log::debug → qa
     ^cwtracing::debug!<Esc>
     f"i msg = <Esc>
     f,a index = <Esc>j
     q
     7@a

C-5: /use crate.*Error; → ci{ → Error, Result<Esc>
     n.n.
```

</details>

---

## 別解

<details>
<summary>別解を見る（上達したら試してみよう）</summary>

- マクロを `"a` レジスタに保存しておけば `:w` してもクリアされない（セッション内）
- `:5,12normal @a` でマクロを5〜12行目に適用することもできる
- `qA`（大文字）でレジスタ `a` のマクロに**追記**できる

</details>
