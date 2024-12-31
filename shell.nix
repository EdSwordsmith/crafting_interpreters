{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    rustc
    rustfmt
    clippy
    zig
    # Challenges
    gnumake
    temurin-bin
    clang
  ];
}
