# ミッション 2 — 分割ウィンドウとタブで複数ファイルを同時編集

**難易度**: ★★☆  
**目安時間**: 10分

---

## 前提

`mission-2/project/` に `main.rs`・`lib.rs`・`Cargo.toml` が置いてある。

---

## 課題

1. `main.rs` を開き、`:vsp lib.rs` で左右に分割する
2. 左ペイン（`main.rs`）に `lib.rs` の関数を呼び出すコードを追加する。右ペイン（`lib.rs`）を参照しながら書く
3. `Ctrl+w h` / `Ctrl+w l` でペインを行き来する
4. `:tabnew Cargo.toml` で新しいタブに `Cargo.toml` を開く
5. `gt` / `gT` でタブを切り替える
6. `Cargo.toml` の `name` フィールドの値を確認し、`main.rs` のコメントにそのプロジェクト名を記載する
7. `:tabclose` でタブを閉じ、`:only` で分割を解消してウィンドウを1つにする

## 制約

- マウス禁止
- ウィンドウ間の移動は `Ctrl+w h/j/k/l` のみ（矢印キー禁止）

## ゴール

`main.rs` に `lib.rs` の関数を呼び出すコードと、プロジェクト名のコメントが追加されていること。

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- `Ctrl+w w` で次のウィンドウへローテーション（h/j/k/l を覚えるまでの繋ぎに）
- `:ls` はどのウィンドウからでもバッファ全体を確認できる
- タブ上部の表示（`main.rs | Cargo.toml`）でタブを視覚的に確認できる
- `:tabonly` でほかのタブを全部閉じることもできる

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```
:e project/src/main.rs
:vsp project/src/lib.rs
Ctrl+w h    （main.rs 側へ）
（lib.rs を参照しながら main.rs を編集）
:tabnew project/Cargo.toml
（Cargo.toml で name を確認）
gT          （前のタブへ）
Ctrl+w h    （main.rs へ）
（コメントを追加）
:tabclose
:only
:wa
```

</details>
