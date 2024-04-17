{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell rec {
  nativeBuildInputs = [
    pkg-config
    clang
  ];
  buildInputs = [
    glibc
    pipewire
    libclang
    ffmpeg
    alsa-lib
    dbus
    SDL2
    SDL2_gfx
    SDL2_image
  ];

  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
  BINDGEN_EXTRA_CLANG_ARGS = [
    ''-I"${pkgs.llvmPackages.libclang.lib}/lib/clang/${pkgs.llvmPackages.libclang.version}/include"''
    "-I ${pkgs.glibc.dev}/include"
  ];
}
