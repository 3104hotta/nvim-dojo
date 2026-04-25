# ミッション A — ウィンドウサイズ：比率を指定してレイアウトを作る

**難易度**: ★★☆  
**目安時間**: 10分

---

## 課題

以下のレイアウトを、コマンドだけで再現せよ。

```
┌──────────────┬─────────────────────┬──────────┐
│              │                     │          │
│   left.rs    │     center.rs       │ right.rs │
│   （幅20列）  │     （幅50列）       │ （幅15列） │
│              │                     │          │
└──────────────┴─────────────────────┴──────────┘
```

1. `left.rs` を開いた状態から `:vsp center.rs`、さらに `:vsp right.rs` で3ペインを作る
2. `left.rs` のペインを幅20列に調整する（`:vertical resize 20`）
3. `right.rs` のペインを幅15列に調整する（`:vertical resize 15`）
4. `center.rs` は残りを自動的に占める（`Ctrl+w =` で一度均等化してから上記の手順を試す）
5. `Ctrl+w =` で全ウィンドウを均等化する
6. 再び上記の比率に戻す（今度は `Ctrl+w >` / `Ctrl+w <` を使って微調整する）

## 制約

- マウスによるウィンドウサイズ変更は禁止
- 手順5→6 では `Ctrl+w >/<` を使う（`:vertical resize` 禁止）

## ゴール

left: 20列 / right: 15列 のレイアウトが再現できること。
（center の幅は端末幅によって変わるため問わない）

---

## ヒント

<details>
<summary>ヒントを見る</summary>

- `:vertical resize 20` = 現在のウィンドウの幅を20列に指定
- `Ctrl+w =` で均等化した後、`:vertical resize` で絶対値指定すると楽
- `{N}Ctrl+w >` で N 列まとめて広げられる（例: `10Ctrl+w >` で10列増加）
- ターミナル幅を `:echo &columns` で確認できる

</details>

---

## 解答例

<details>
<summary>解答を見る</summary>

```
:e left.rs
:vsp center.rs
:vsp right.rs
Ctrl+w h   （left.rs へ移動）
:vertical resize 20
Ctrl+w l   （right.rs のペインへ）
Ctrl+w l
:vertical resize 15
```

</details>

---

## 別解

<details>
<summary>別解を見る（上達したら試してみよう）</summary>

最小キーストロークへの挑戦:
```
:e left.rs | vsp center.rs | vsp right.rs
Ctrl+w h | :vert res 20 | 2Ctrl+w l | :vert res 15
```

`Ctrl+w |` で現在ウィンドウを最大幅にしてから縮める方法もある。

</details>
