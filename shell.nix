{ pkgs ? import <nixpkgs> {} }:
(pkgs.buildFHSEnv {
  name = "wasmer-env";

  targetPkgs = pkgs: (with pkgs; [
    cmake llvm_18.dev pkg-config libffi libxml2 ninja llvmPackages_18.libclang.dev llvmPackages_18.stdenv llvmPackages_18.clang libz.dev ncurses6.dev
  ]);

  runScript = pkgs.writeShellScript "wasmer-env-run" (''
    export BINDGEN_EXTRA_CLANG_ARGS="$(< ${pkgs.llvmPackages_18.stdenv.cc}/nix-support/libc-crt1-cflags) \
      $(< ${pkgs.llvmPackages_18.stdenv.cc}/nix-support/libc-cflags) \
      $(< ${pkgs.llvmPackages_18.stdenv.cc}/nix-support/cc-cflags) \
      $(< ${pkgs.llvmPackages_18.stdenv.cc}/nix-support/libcxx-cxxflags) \
      -idirafter ${pkgs.llvmPackages_18.stdenv.cc.cc}/lib/clang/18/include \
    "
    if [ -n "$''+''{NIX_SHELL_RUN:+set}" ]; then
      eval "$NIX_SHELL_RUN"
    else
      exec bash
    fi
  '');
}).env
