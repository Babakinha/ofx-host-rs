{ pkgs ? import <nixpkgs> {} }:

let
  openfx-src = pkgs.fetchFromGitHub {
    owner = "AcademySoftwareFoundation";
    repo = "openfx";
    rev = "OFX_Release_1.5.1";
    sha256 = "sha256-qiY5klmGDiU9cqjfNdFsCcNqSBwV0dVZB2ZIsElRBD4=";
  };
in
pkgs.mkShell {
  name = "mreow";

  buildInputs = with pkgs; [
    llvmPackages.libclang
    llvmPackages.clang

    expat
  ];

  shellHook = ''
    export OPENFX_DIR="${openfx-src}"
    export LD_LIBRARY_PATH=${pkgs.stdenv.cc.cc.lib}/lib/
    export LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib"
    export BINDGEN_EXTRA_CLANG_ARGS="-isystem ${pkgs.llvmPackages.clang}/resource-root/include -isystem ${pkgs.stdenv.cc.cc}/include/c++/${pkgs.stdenv.cc.version} -isystem ${pkgs.stdenv.cc.cc}/include/c++/${pkgs.stdenv.cc.version}/x86_64-unknown-linux-gnu"
    export BINDGEN_EXTRA_CLANG_ARGS="$BINDGEN_EXTRA_CLANG_ARGS $(cat ${pkgs.stdenv.cc}/nix-support/libc-cflags) $(cat ${pkgs.stdenv.cc}/nix-support/cc-cflags)"
    export BINDGEN_EXTRA_CLANG_ARGS="$BINDGEN_EXTRA_CLANG_ARGS -isystem ${pkgs.expat.dev}/include"
  '';
}
