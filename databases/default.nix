{ lib, stdenv, lua5_4, luajit }:

stdenv.mkDerivation {
  pname = "metrics-ao";
  version = "0.1.0";

  src = ./.;

  buildInputs = [
    lua5_4
    luajit
  ];

  installPhase = ''
    mkdir -p $out/bin
    cp metrics_ao.lua $out/bin/
    chmod +x $out/bin/metrics_ao.lua
  '';
}
