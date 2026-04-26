# ミッション 2 — Ctrl+d/u・H/M/L でスクロール移動

**難易度**: ★★☆
**目安時間**: 5〜10分

---

## 課題

`exercise.rs` を nvim で開き、以下4箇所の編集を行え。
ファイルは190行超あり、編集箇所が **先頭・中央・末尾に散らばっている** ため、
スクロール系コマンドを使わないと時間がかかる。

1. **ファイル末尾** `pub fn cleanup` 関数内、`Ok(())` の直前に
   `// cleanup complete` というコメント行を追加する
2. **ファイル先頭** `use std::io;` の次の行に
   `use std::collections::BTreeMap;` を追加する
3. **中央上付近** `impl Default for Config` 内の
   `data_dir: PathBuf::from("/var/data"),` を `"/tmp/data"` に変更する
4. **中央付近** `impl Server` の `start` メソッド内、`println!` の文字列
   `"Starting server on {}:{}"` を `"Server starting on {}:{}"` に変更する

## 制約

- `j` / `k` の連打は禁止（1回ずつは可）
- `Ctrl+d`・`Ctrl+u`・`H`・`M`・`L`・`gg`・`G` を積極的に使うこと

## ゴール

`goal.rs` と同一の状態にすること。

```
1. （末尾）pub fn cleanup ... {
       store.cache.clear();
       server.stop();
+      // cleanup complete
       Ok(())
   }

2. （先頭）use std::io;
+  use std::collections::BTreeMap;
   use std::collections::HashMap;

3. （中央上）data_dir: PathBuf::from("/var/data"),
   →           data_dir: PathBuf::from("/tmp/data"),

4. （中央）"Starting server on {}:{}"
   →     "Server starting on {}:{}"
```

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- `gg` でファイル先頭へ、`G` で末尾へ一瞬で移動できる
- `Ctrl+d` で半画面スクロール → 目的地が近づいたら `H`/`M`/`L` で画面内の行へジャンプ
- `/cleanup` `/Default` `/Starting` のように検索を使うと一発でジャンプできる
- `zz` でカーソル行を画面中央に寄せると見やすい

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```
1. G              → ファイル末尾へ
   /Ok(())        → cleanup の Ok(()) を検索
   O              → 上に新行を作成
   // cleanup complete<Esc>

2. gg             → ファイル先頭へ
   j              → use std::io; の次の行へ
   O              → 上に新行を作成（または現在行で o の代わりに O）
   use std::collections::BTreeMap;<Esc>

3. /var/data      → 検索でジャンプ
   ciw            → "var" を変更（または cw, f/ で位置調整）
   tmp<Esc>

4. /Starting      → 検索でジャンプ
   ct{            → "{" の手前まで変更
   Server starting on <Esc>
```

</details>

---

## 別解

<details>
<summary>別解を見る（上達したら試してみよう）</summary>

- `50%` でファイルの50%位置（中央付近）へジャンプ → `H`/`M`/`L` で微調整
- `:42` のように行番号を直接指定して移動
- `Ctrl+f` / `Ctrl+b` で1画面スクロール（大きいファイルに有効）
- `*` でカーソル位置の単語を検索

</details>
