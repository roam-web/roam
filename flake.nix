{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in with pkgs; rec {
        platformInputs = [ pkg-config ] ++ lib.optionals (stdenv.isDarwin)
          (with darwin.apple_sdk.frameworks; [
            AppKit
            CoreGraphics
            CoreServices
            CoreText
            Foundation
            OpenGL
          ]) ++ lib.optionals (stdenv.isLinux) [
            udev
            alsa-lib
            vulkan-loader
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            libxkbcommon
            wayland
          ];

        devShells.default = mkShell rec {
          buildInputs = [
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" "rust-analyzer" ];
            })
          ] ++ platformInputs;

          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
      });
}
