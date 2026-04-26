# ミッション 3 — レジスタを使い分けたヤンク・ペースト

**難易度**: ★★★
**目安時間**: 10分

---

## 課題

`exercise.rs` には複数の関数がある。レジスタを意識してコピー・ペースト操作を行え。

1. **関数全体の複製** `fn alpha` の関数本体（`pub fn alpha(...)` から閉じ `}` まで）を `fn gamma` の後ろに複製し、複製した関数の名前を `alpha2` に変更する
2. **行ヤンク・ペースト** `fn delta` 内の `println!("delta: {}", x);` 行をヤンクし、`fn gamma` の本体冒頭（`x * x` の前の行）に貼り付ける（元の delta 側はそのまま残す）
3. **ヤンクレジスタ `"0`** `let msg = "hello world";` の中身 `hello world` を `yi"` でヤンクする → ファイル末尾に新しい行を作って `// ` と入力 → そこに `"0p` で `hello world` を貼り付け、`// hello world` という行を作る
4. **システムクリップボード `"+`** `pub fn public_api(input: &str) -> Result<Vec<i32>, String>` のシグネチャ行を `"+yy` でシステムクリップボードにコピーする（ファイルへの変更は不要、クリップボードに入っていれば OK）

## 制約

- 課題1は `V` で行選択 → `%` で対応する `}` までジャンプ → `y` でヤンク → 移動 → `p` でペーストの流れで行うこと
- 課題3は `yi"` でヤンクしたあとに `dd` 等でデフォルトレジスタを上書きしてもよい（`"0` には残ることを体験する）
- 課題4は `"+yy` または `"+y$` を使う

## ゴール

`goal.rs` と同一の状態にすること（課題4はクリップボードに入っていれば OK）。

```
1. （fn gamma の後）
+  pub fn alpha2(x: i32) -> i32 {
+      let doubled = x * 2;
+      let shifted = doubled + 1;
+      shifted
+  }

2. （fn gamma 本体冒頭）
   pub fn gamma(x: i32) -> i32 {
+      println!("delta: {}", x);
       x * x
   }

3. （ファイル末尾）
+  // hello world

4. （クリップボード内容）
   pub fn public_api(input: &str) -> Result<Vec<i32>, String>
```

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- `"0p` = ヤンクレジスタ（`"0`）から貼り付け。`d` や `c` で上書きされない
- `"+y` = システムクリップボード（`"+`）へヤンク
- `:reg` でレジスタの内容を確認できる
- 関数全体のヤンク: 関数先頭行で `V` → `%`（対応する `}` まで） → `y`
- 関数名のリネーム: 複製先で `cw` → 新しい名前を入力

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```
課題1:
  /fn alpha       → V%y          → 関数全体ヤンク
  /fn gamma       → }            → gamma の閉じ } の次へ
  p               → ペースト
  /fn alpha       → n            → 複製した方へジャンプ
  fw cw           → 関数名へ → alpha2<Esc>

課題2:
  /println!\("delta → yy        → 行ヤンク
  /fn gamma       → j            → 本体冒頭へ
  P               → 上に貼り付け

課題3:
  /hello world    → yi"          → "hello world" の中身ヤンク
  （任意で dd 等） → デフォルトレジスタ上書きしても "0 は無事
  G o // <Esc>    → 末尾に // の行を作成
  "0p             → ヤンクレジスタからペースト

課題4:
  /pub fn public_api → "+yy
```

</details>

---

## 別解

<details>
<summary>別解を見る（上達したら試してみよう）</summary>

- 名前つきレジスタ `"a` 〜 `"z` に保存して使い回す: `"ayy` → `:reg a` で確認 → `"ap`
- `"Ayy` で大文字のレジスタ名を使うと追記できる（上書きでなく）
- `:let @a = @+` でクリップボードの内容をレジスタ a にコピー

</details>

---

## 演習リセット

演習で `exercise.rs` を編集した後、コミット前に元の状態へ戻すこと。
そうしないと次回の演習で「最初から `goal.rs` と同じ」になってしまう。

```bash
# このミッションだけリセット
scripts/reset.sh 03-verb-noun/mission-3

# 全ミッションをまとめてリセット
scripts/reset.sh
```
