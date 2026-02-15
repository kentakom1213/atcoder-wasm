{
  description = "atcoder-wasm development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);
    in
    {
      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
          rustToolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
            toolchain.default.override {
              extensions = [ "rust-src" ];
              targets = [ "wasm32-wasip1" ];
            });
        in
        {
          default = pkgs.mkShell {
            packages = [
              rustToolchain
              pkgs.wasm-tools
              pkgs.binaryen
            ];
          };
        });

      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
          rustToolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
            toolchain.default.override {
              extensions = [ "rust-src" ];
              targets = [ "wasm32-wasip1" ];
            });
        in
        {
          build = pkgs.writeShellApplication {
            name = "build-wasm";
            runtimeInputs = [
              rustToolchain
              pkgs.wasm-tools
              pkgs.binaryen
              pkgs.coreutils
              pkgs.gawk
            ];
            text = ''
              set -eu

              if [ "$#" -ne 1 ]; then
                echo "usage: nix run .#build -- <bin-name>"
                exit 1
              fi

              RUSTFLAGS="\
                --remap-path-prefix $HOME=~ \
                --remap-path-prefix $(pwd)=. \
              "
              export RUSTFLAGS

              BIN_NAME="$1"
              OUT_NAME="results/$BIN_NAME"
              TARGET="wasm32-wasip1"

              mkdir -p results

              cargo build \
                -Z unstable-options \
                -Z build-std=std \
                -r --bin "$BIN_NAME" --target "$TARGET"

              wasm-opt -Oz \
                --enable-bulk-memory-opt \
                "target/$TARGET/release/$BIN_NAME.wasm" \
                -o "$OUT_NAME.wasm"

              wasm-tools strip "$OUT_NAME.wasm" -o "$OUT_NAME.wasm"
              wasm-tools print "$OUT_NAME.wasm" > "$OUT_NAME.wat"

              WAT_BYTES=$(wc -c < "$OUT_NAME.wat" | tr -d ' ')
              WAT_KIB=$(awk "BEGIN {printf \"%.2f\", $WAT_BYTES/1024}")

              echo "done:"
              echo "  $OUT_NAME.wat  : ''${WAT_KIB} KiB"
            '';
          };

          default = self.packages.${system}.build;
        });

      apps = forAllSystems (system: {
        build = {
          type = "app";
          program = "${self.packages.${system}.build}/bin/build-wasm";
        };
        default = self.apps.${system}.build;
      });
    };
}
