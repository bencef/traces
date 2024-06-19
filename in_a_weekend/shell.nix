{pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  packages = with pkgs; [ rust-analyzer cargo rustc rustfmt clippy ];
}
