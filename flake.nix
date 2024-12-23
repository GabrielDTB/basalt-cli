{
  description = "Basalt dev shell";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = with pkgs; mkShell.override{
          stdenv = stdenvAdapters.useMoldLinker clangStdenv;
        } {
          buildInputs = [
            openssl
            pkg-config
            typst
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions = [
                "rust-src"
                "rust-analyzer"
                "rustfmt"
              ];
            }))
          ];
        };
      }
    );
}

