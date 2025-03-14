{ lib, stdenv, lua5_4, luajit }:

stdenv.mkDerivation {
  pname = "quantum-db-ao";
  version = "0.1.0";

  src = ./.;

  buildInputs = [
    lua5_4
    luajit
  ];

  installPhase = ''
    mkdir -p $out/bin
    cp quantum_ao.lua $out/bin/
    chmod +x $out/bin/quantum_ao.lua
  '';
}
