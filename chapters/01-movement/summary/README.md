# 章末総合演習 — 移動コマンドを駆使してコードリーディング

**難易度**: ★★★
**目安時間**: 15〜20分

---

## シナリオ

小規模な HTTP クライアントライブラリのコードレビューを任された。
`exercise.rs`（180行超）を読み解きながら、指定された変更をすべて **`hjkl` なしで** 完了させよ。
編集箇所はファイル全体に散らばっているので、章で学んだ移動コマンドを総動員すること。

## 課題

以下の7項目を完了させること:

1. **先頭付近** `use std::io;` の次の行に `use std::path::PathBuf;` を追加する
2. **先頭コメント** `// HTTP client library` を `// HTTP client (review pass)` に変更する
3. **中央上** `HttpClient::new` 内の `"nvim-dojo-client/1.0"` を `"nvim-dojo-client/2.0"` に変更する
4. **中央** `pub fn get` の `println!("GET {}", url);` を `println!("[GET] {}", url);` に変更する
5. **中央** `pub fn delete` の `println!("DELETE {}", url);` を `println!("[DELETE] {}", url);` に変更する
6. **中央下** `HttpError::Unknown` の Display 実装で `"unknown error"` を `"unknown HTTP error"` に変更する
7. **末尾付近** `build_query_string` 関数の閉じ `}` の次に `// reviewed by: <your-name>` の行を追加する

## 制約

- `hjkl` 移動禁止
- `f`・`t`・`w`・`b`・`e`・`0`・`$`・`gg`・`G`・`H`・`M`・`L`・`Ctrl+d`・`Ctrl+u` をフル活用
- `/`・`?` での検索もOK

## ゴール

`goal.rs` と同一の状態にすること。

```
1. （先頭）  use std::io;
+            use std::path::PathBuf;

2. （先頭）  // HTTP client library
   →         // HTTP client (review pass)

3. （中央上）"nvim-dojo-client/1.0"
   →         "nvim-dojo-client/2.0"

4. （中央）  println!("GET {}", url);
   →         println!("[GET] {}", url);

5. （中央）  println!("DELETE {}", url);
   →         println!("[DELETE] {}", url);

6. （中央下）"unknown error"
   →         "unknown HTTP error"

7. （末尾）  pub fn build_query_string(...) { ... }
+            // reviewed by: <your-name>
```

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- `gg` / `G` でファイル先頭・末尾へ一発移動
- 検索 `/use std::io` `/GET` `/DELETE` `/unknown error` でジャンプが速い
- `Ctrl+d` で半画面スクロールしながら全体感を掴む → `H`/`M`/`L` で画面内の行へ
- `ci"` でクォート内の文字列を変更
- `ct,` `ct)` で「次の `,` `)` 手前まで」を変更
- `o` でカーソル行の下、`O` で上に新しい行を作成

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```
1. gg            → ファイル先頭
   /std::io      → 検索でジャンプ
   o             → 下に新行
   use std::path::PathBuf;<Esc>

2. /HTTP client  → 検索
   f l           → "library" の l へ
   ct<Enter>     → 行末まで変更（または ci( で括弧内も）
   (review pass)<Esc>
   # 簡単には: 行頭から `cf;` で行末まで変更しても可

3. /1.0          → 検索でジャンプ
   r 2           → '1' を '2' に置換 （あるいは f1 → r2）

4. /GET          → 検索
   f"            → '"' へ
   ci"           → クォート内変更
   [GET] {}<Esc>

5. /DELETE       → 検索
   f"            → '"' へ
   ci"           → クォート内変更
   [DELETE] {}<Esc>

6. /unknown error → 検索
   ci"           → クォート内変更
   unknown HTTP error<Esc>

7. /build_query_string → 検索
   %             → 関数本体の閉じ '}' へジャンプ
   o             → 下に新行
   // reviewed by: <your-name><Esc>
```

</details>

---

## 振り返り

- 7箇所のうち、最も時間がかかったのはどれか？
- `hjkl` を使いたくなった瞬間はあったか？そのときどのコマンドが代替になったか？
- 検索（`/`）と画面内ジャンプ（`H`/`M`/`L`）、どちらをよく使ったか？

---

## 演習リセット

演習で `exercise.rs` を編集した後、コミット前に元の状態へ戻すこと。
そうしないと次回の演習で「最初から `goal.rs` と同じ」になってしまう。

```bash
# このミッションだけリセット
scripts/reset.sh 01-movement/summary

# 全ミッションをまとめてリセット
scripts/reset.sh
```
