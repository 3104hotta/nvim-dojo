# ミッション 2 — Ctrl+d/u・H/M/L でスクロール移動

**難易度**: ★★☆  
**目安時間**: 5〜10分

---

## 課題

`exercise.rs` を nvim で開き、以下の編集を行え。
ファイルは100行超あるため、スクロール系コマンドを使わないと時間がかかる。

1. ファイル末尾の `fn cleanup` 関数内にある `unimplemented!()` を `Ok(())` に変更する
2. ファイル先頭の `use` 宣言に `use std::time::Duration;` を追加する
3. ファイル中央付近の `struct Config` の `timeout` フィールドの型を `u32` から `Duration` に変更する
4. 各 `impl` ブロックの先頭行（`impl` キーワードの行）に移動して確認する

## 制約

- `j` / `k` の連打は禁止（1回ずつは可）
- `Ctrl+d`・`Ctrl+u`・`H`・`M`・`L`・`gg`・`G` を積極的に使うこと

## ゴール

`goal.rs` と同一の状態にすること。

```
before: use std::io;
        （追加）
after:  use std::io;
        use std::time::Duration;

before: timeout: u32,
after:  timeout: Duration,

before:     unimplemented!()
after:      Ok(())
```

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- `gg` でファイル先頭へ、`G` で末尾へ一瞬で移動できる
- `Ctrl+d` で半画面スクロール → 目的地が近づいたら `H`/`M`/`L` で画面内の行へジャンプ
- `/unimplemented` で検索すれば一発でジャンプできる
- `zz` でカーソル行を画面中央に寄せると見やすい

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```
1. G              → ファイル末尾へ
   /unimplemented → 検索でジャンプ
   ciw            → 単語変更
   Ok(())<Esc>

2. gg             → ファイル先頭へ
   j              → use 行の次の行へ
   O              → 上に新行を作成
   use std::time::Duration;<Esc>

3. /timeout       → 検索
   f u            → 'u' (u32) へジャンプ
   cw             → 単語変更
   Duration<Esc>
```

</details>

---

## 別解

<details>
<summary>別解を見る（上達したら試してみよう）</summary>

- `50%` でファイルの50%位置（中央付近）へジャンプ → `H`/`M`/`L` で微調整
- `:42` のように行番号を直接指定して移動
- `Ctrl+f` / `Ctrl+b` で1画面スクロール（大きいファイルに有効）

</details>
