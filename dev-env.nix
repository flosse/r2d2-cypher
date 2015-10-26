let
  pkgs    = import <nixpkgs> {};
  stdenv  = pkgs.stdenv;
  lib     = pkgs.lib;

in rec {
  devEnv = stdenv.mkDerivation rec {
    name = "r2d2-cypher-dev-env";
    src = ./.;
    buildInputs = with pkgs; [
      git
      rustPlatform.rustc
      rustfmt
      cargo
      openssl
    ];
  };
}
