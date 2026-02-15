#!/bin/sh

set -eu

if [ $# -ne 1 ]; then
  echo "usage: $0 <bin-name>"
  exit 1
fi

BIN_NAME="$1"
OUT_NAME="results/$BIN_NAME"
TARGET="wasm32-wasip1"

# build
cargo build -r --bin "$BIN_NAME" --target "$TARGET"

# strip
wasm-tools strip \
  "target/$TARGET/release/$BIN_NAME.wasm" \
  -o "$OUT_NAME.wasm"

# 最適化
# wasm-opt で最適化 + strip 相当
wasm-opt -Oz --enable-bulk-memory-opt \
  "target/$TARGET/release/$BIN_NAME.wasm" \
  -o "$OUT_NAME.wasm"

# wat 出力
wasm-tools print "$OUT_NAME.wasm" > "$OUT_NAME.wat"


# サイズ取得（bytes）
WAT_BYTES=$(wc -c < "$OUT_NAME.wat" | tr -d ' ')

# KiB 変換
WAT_KIB=$(awk "BEGIN {printf \"%.2f\", $WAT_BYTES/1024}")

echo "done:"
echo "  $OUT_NAME.wat  : ${WAT_KIB} KiB"
