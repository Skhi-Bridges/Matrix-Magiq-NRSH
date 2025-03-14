# default.nix
{ pkgs ? import <nixpkgs> {} }:

with pkgs;

let
  pythonEnv = python39.withPackages (ps: with ps; [
    streamlit
    qiskit
    numpy
    lupa
    torch
    transformers
    pillow
    (buildPythonPackage rec {
      pname = "matrix-magiq";
      version = "0.1.0";
      src = ./.;
      propagatedBuildInputs = [
        streamlit
        qiskit
        numpy
        lupa
      ];
    })
  ]);

  luaEnv = lua53Packages.withPackages (ps: with ps; [
    luafilesystem
    luasocket
    lpeg
    luajson
  ]);

in mkShell {
  buildInputs = [
    pythonEnv
    luaEnv
    sqlite
    annoy
    hdf5
    rocksdb
    lmdb
  ];

  shellHook = ''
    export PYTHONPATH=$PYTHONPATH:$PWD
    export LUA_PATH="$PWD/?.lua;$PWD/?/init.lua;$LUA_PATH"
    export LUA_CPATH="$PWD/?.so;$LUA_CPATH"
  '';
}
