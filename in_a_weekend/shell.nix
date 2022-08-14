{pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  packages = with pkgs; [ rls rust-analyzer cargo rustc rustfmt clippy ];
}
