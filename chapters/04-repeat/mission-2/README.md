# ミッション 2 — マクロで複数行を一括変換

**難易度**: ★★☆  
**目安時間**: 10分

---

## 課題

`exercise.rs` には同じ構造の行が10行並んでいる。
マクロを記録して一括変換せよ。

**変換前（各行）**:
```rust
    ("key_name", "default_value"),
```

**変換後（各行）**:
```rust
    Config::new("key_name", "default_value"),
```

さらに別のセクションでは:

**変換前**:
```rust
let result_1 = compute(1);
let result_2 = compute(2);
...
let result_8 = compute(8);
```

**変換後**:
```rust
let result_1 = compute(1).unwrap_or(0);
let result_2 = compute(2).unwrap_or(0);
...
let result_8 = compute(8).unwrap_or(0);
```

## 制約

- マクロ（`q{a}` で記録 → `@a` で再生）を使うこと
- マクロの末尾に `j` を含めて `{N}@@` で N 行まとめて適用すること

## ゴール

`goal.rs` と同一の状態にすること。

---

## ヒント

<details>
<summary>ヒントを見る</summary>

**1つ目のマクロ（Config::new 追加）**:
1. 最初の `("key_name"` の行へ移動
2. `qa` でマクロ記録開始
3. `^` → `i` → `Config::new` → `Esc` → `j` → `q` で記録終了
4. `9@a` で残り9行に適用

**2つ目のマクロ（.unwrap_or(0) 追加）**:
1. 最初の `compute(1)` 行へ
2. `qb` で記録
3. `f;` → `i` → `.unwrap_or(0)` → `Esc` → `j` → `q`
4. `7@b`

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```
マクロ1:
  qa
  ^iConfig::new<Esc>j
  q
  9@a

マクロ2:
  qb
  f;i.unwrap_or(0)<Esc>j
  q
  7@b
```

</details>

---

## 別解

<details>
<summary>別解を見る（上達したら試してみよう）</summary>

- `:%s/^    (/    Config::new(/ ` で一括置換（マクロが不要になるケース）
- `:%s/compute(\d)/&.unwrap_or(0)/g` で正規表現置換
- マクロの中で `*`（単語検索）や `n`（次のマッチ）を使うとより汎用的なマクロになる

</details>
