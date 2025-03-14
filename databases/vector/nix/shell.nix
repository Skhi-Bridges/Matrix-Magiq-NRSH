# shell.nix
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    lua5_4
    luajit
    luarocks
    # Database dependencies
    redis
    postgresql
    neo4j
    # Vector store dependencies
    python39Packages.annoy
    python39Packages.faiss
    # Development tools
    nodejs
    yarn
  ];

  shellHook = ''
    export PATH=$PATH:$PWD/node_modules/.bin
    export LUA_PATH="$PWD/lua/?.lua;$PWD/lua/?/init.lua;;"
    export LUA_CPATH="$PWD/lua/?.so;;"
  '';
}
