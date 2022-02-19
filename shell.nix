# RUST

let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in

with nixpkgs;

mkShell {
  name = "rust";
    nativeBuildInputs = [
    pkgconfig
    clang lld # To use lld linker
  ];
  #buildInputs = [ rustc cargo cargo-make openssl rustup lld rust-analyzer rustfmt clippy pkgconfig ];
  buildInputs = [
  x11 xorg.libXcursor xorg.libXrandr xorg.libXi
  vulkan-tools vulkan-headers vulkan-loader vulkan-validation-layers
  udev pkgconfig alsaLib
  cargo-make watchexec clippy rustfmt rust-analyzer openssl lld pkgconfig
  lutris
  (latest.rustChannels.stable.rust.override {
    targets = ["wasm32-unknown-unknown" "x86_64-unknown-linux-gnu"];
  })
  latest.rustChannels.stable.cargo
  latest.rustChannels.stable.rust-src
  latest.rustChannels.stable.rustc
  ];

  shellHook = ''
    export PATH=$PATH:~/.cargo/bin
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
    udev alsaLib vulkan-loader
  ]}"
  '';
}
