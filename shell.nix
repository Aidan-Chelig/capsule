# RUST
{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell {
  name = "rust";
  buildInputs = [ rustfmt rustc cargo clippy pkgconfig udev alsaLib lutris
    x11 xorg.libXcursor xorg.libXrandr xorg.libXi
    vulkan-tools vulkan-headers vulkan-loader vulkan-validation-layers];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
