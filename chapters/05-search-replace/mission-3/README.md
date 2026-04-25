# ミッション 3 — vimgrep + quickfix + `:cdo` で複数ファイル一括置換

**難易度**: ★★★  
**目安時間**: 15分

---

## 前提

このミッションは `mission-3/` ディレクトリ内に複数の Rust ファイルがある。
まず nvim を `mission-3/` ディレクトリ内で起動すること。

```bash
nvim src/main.rs
```

---

## 課題

`src/` 以下の全 `.rs` ファイルに対して操作を行え。

1. `vimgrep` で `deprecated_api` を含む行を全ファイルから検索し、quickfix ウィンドウで一覧する
2. quickfix を使って各マッチを確認しながら `deprecated_api` → `new_api` に変更する（`:cdo` を使う）
3. `vimgrep` で `TODO` コメントを検索し、内容を quickfix ウィンドウで確認する
4. quickfix の2番目と4番目のエントリだけに移動して確認する（`:cc 2`・`:cc 4`）
5. `:cdo` で全 `TODO` を `FIXME` に変更して保存する（`:cdo s/TODO/FIXME/g | update`）

## 制約

- ファイルを1つずつ開いて検索・置換するのは禁止
- `:vimgrep` + `:cdo` の組み合わせを使うこと

## ゴール

`src/` 以下の全ファイルで `deprecated_api` → `new_api` の置換が完了し、`TODO` が `FIXME` になっていること。

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- `:vimgrep /pattern/ **/*.rs` で再帰的に検索
- `:copen` で quickfix ウィンドウを開く
- quickfix ウィンドウ内で `Enter` でそのファイルの該当行へジャンプ
- `:cdo {cmd}` = quickfix の各エントリに対してコマンドを実行
- `:cdo s/old/new/g | update` で置換 + 上書き保存を各ファイルで実行

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```
:vimgrep /deprecated_api/ **/*.rs
:copen
:cdo s/deprecated_api/new_api/g | update

:vimgrep /TODO/ **/*.rs
:copen
:cc 2
:cc 4
:cdo s/TODO/FIXME/g | update
```

</details>

---

## 別解

<details>
<summary>別解を見る（上達したら試してみよう）</summary>

- `cfdo` = quickfix の各**ファイル**に対して1回だけコマンドを実行（重複処理を避けられる）
  `:cfdo %s/TODO/FIXME/g | update`
- `:args **/*.rs | argdo %s/old/new/g | update` でも複数ファイル置換できる
- quickfix を閉じた後も `:cn`・`:cp` でナビゲートできる

</details>
