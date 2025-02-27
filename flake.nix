{
  description = "Wasm Proxy";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1.*.tar.gz";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    overlays = [
      rust-overlay.overlays.default
      (final: prev: {
        rustToolchain = prev.rust-bin.stable.latest.default.override {
          extensions = ["rust-src" "rustfmt"];
          targets = ["wasm32-unknown-unknown" "wasm32-wasi"];
        };
      })
    ];
    supportedSystems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
    forEachSupportedSystem = f:
      nixpkgs.lib.genAttrs supportedSystems (system:
        f {
          pkgs = import nixpkgs {inherit overlays system;};
        });
  in {
    devShells = forEachSupportedSystem ({pkgs}: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          httpie

          rustToolchain
          openssl
          pkg-config
          cargo-deny
          cargo-edit
          cargo-watch
          cargo-component
          rust-analyzer
          wasm-tools
          wit-bindgen
          wasmtime

          (pkgs.writeShellScriptBin "build-component" ''
            #!/usr/bin/env bash
            cd $(git rev-parse --show-toplevel)
            cargo component build --release --package amplitude
          '')

          (pkgs.writeShellScriptBin "run-proxy" ''
            #!/usr/bin/env bash
            cd $(git rev-parse --show-toplevel)
            cargo run --package proxy
          '')
        ];

        shellHook = ''
          export CARGO_HOME=$PWD/.cargo
          export PATH=$CARGO_HOME/bin:$PATH
          export WASM_PATH=$PWD/target/wasm32-wasi/release
        '';
      };
    });
  };
}
