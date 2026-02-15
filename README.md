# atcoder-wasm

Nix ベースで Rust (nightly) + wasm ツールチェーンを使って AtCoder 向け wasm をビルドするプロジェクトです。

## 使い方

### 1. 開発シェルに入る

```sh
nix develop
```

`nix-command` / `flakes` が無効な環境では、以下のように有効化して実行してください。

```sh
nix --extra-experimental-features "nix-command flakes" develop
```

### 2. 通常の開発

開発シェル内で通常どおり `cargo` を使えます。

```sh
cargo test
cargo run --bin abc301-a
```

### 3. wasm ビルド

`nix run` で `build` app を実行します。  
`<bin-name>` には `src/bin/*.rs` のファイル名（拡張子なし）を指定します。

```sh
nix run .#build -- <bin-name>
```

例:

```sh
nix run .#build -- abc301-a
```

ビルド後の成果物:
- `results/<bin-name>.wasm`
- `results/<bin-name>.wat`

### 4. `build.sh` を使う場合

`build.sh` は互換ラッパーで、中で `nix run .#build` を呼びます。

```sh
./build.sh <bin-name>
```
