# ミッション 3 — マージコンフリクトを vimdiff で解消する

**難易度**: ★★★  
**目安時間**: 15分

---

## 前提

`mission-3/project/` は2つのブランチがあり、マージコンフリクトが発生している状態。

```bash
cd mission-3/project
git status   # コンフリクト中のファイルを確認
```

---

## 課題

`conflicted.rs` のコンフリクトを nvim + vimdiff で解消し、コミットせよ。

**コンフリクトの内容**（事前確認不要）:
- `main` ブランチ: `fn validate` の引数に `strict: bool` を追加した
- `feature` ブランチ: `fn validate` の戻り値を `bool` から `Result<(), String>` に変更した
- 両方の変更を取り込んだ実装が正解

## 手順

1. `nvim conflicted.rs` でコンフリクトファイルを開く
2. `/<<<` でコンフリクトマーカーを検索する
3. コンフリクト箇所を読んで、両方の変更を取り込んだコードを書く
4. コンフリクトマーカー（`<<<<<<<`・`=======`・`>>>>>>>`）を削除する
5. `:!cargo check` でコードが正しいことを確認する
6. `:!git add %` → ターミナルで `git commit` を実行する

## 制約

- vimdiff の `do`/`dp` を使う3-way mergetool は使わない（手動解消の練習）
- コンフリクトマーカーは `dd` で1行ずつ削除すること

## ゴール

`goal.rs` と同一の内容になっており、`git status` がクリーンになっていること。

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- コンフリクトマーカーの構造:
  ```
  <<<<<<< HEAD（現在のブランチの変更）
  自分の変更
  =======
  相手の変更
  >>>>>>› feature（マージしようとしたブランチ）
  ```
- 両方の変更を取り込む場合は、マーカーを削除して2つの変更をつなぐ
- `]c` / `[c` は vimdiff モードでのみ使える。通常は `/<<<` で検索する

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```
/<<<      （コンフリクトマーカーへ）
dd        （<<<< 行を削除）
（HEADの変更内容を確認）
/=======  （区切り行へ）
dd        （===== 行を削除）
（feature の変更内容を確認）
/>>>>>>>  （終端マーカーへ）
dd        （>>>> 行を削除）
（両方の変更を統合する形に編集）
:!cargo check
:!git add %
:sp | terminal
i
git commit -m "resolve merge conflict: combine validate changes"
Ctrl+\ Ctrl+n
```

</details>
