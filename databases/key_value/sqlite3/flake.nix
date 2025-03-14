{
  description = "AO Lua Environment Manager";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    
    # Lua packages
    luajit = {
      url = "github:LuaJIT/LuaJIT";
      flake = false;
    };
    
    # Development tools
    zerobrane-studio = {
      url = "github:pkulchenko/ZeroBraneStudio";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        
        # Custom Lua environment
        luaEnv = pkgs.lua54Packages.withPackages (ps: with ps; [
          # Core packages
          luafilesystem
          luasocket
          lpeg
          luajson
          luarocks
          
          # Database interfaces
          lsqlite3
          luasql-sqlite3
          
          # Additional utilities
          penlight
          inspect
          argparse
        ]);

      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Lua environment
            luaEnv
            lua54
            luarocks
            
            # Development tools
            zerobrane-studio
            
            # Build tools
            gcc
            cmake
            pkg-config
          ];

          shellHook = ''
            # Lua paths
            export LUA_PATH="./?.lua;./?/init.lua;./qsr-core/?.lua;./ao-processes/?.lua"
            export LUA_CPATH="./?.so;./?.dylib"
            
            # Project paths
            export QSR_ROOT="$PWD/qsr-core"
            export AO_PROCESSES="$PWD/ao-processes"
            export HDR_ROOT="$PWD/hdr"
            
            # Lua rocks paths
            export LUAROCKS_CONFIG="$PWD/config/luarocks_config.lua"
            
            echo "AO Lua Environment Active"
            echo "Available commands:"
            echo "  - luarocks install <package>  # Install Lua package"
            echo "  - zbstudio                    # Launch ZeroBrane Studio"
            echo "  - lua                         # Start Lua REPL"
          '';
        };

        # Lua package definitions
        packages = {
          qsr-core = pkgs.stdenv.mkDerivation {
            name = "qsr-core";
            src = ./qsr-core;
            buildInputs = [ luaEnv ];
            installPhase = ''
              mkdir -p $out/lib/lua/${pkgs.lua54.luaversion}
              cp -r . $out/lib/lua/${pkgs.lua54.luaversion}/qsr
            '';
          };

          ao-processes = pkgs.stdenv.mkDerivation {
            name = "ao-processes";
            src = ./ao-processes;
            buildInputs = [ luaEnv ];
            installPhase = ''
              mkdir -p $out/lib/lua/${pkgs.lua54.luaversion}
              cp -r . $out/lib/lua/${pkgs.lua54.luaversion}/ao
            '';
          };
        };
      }
    );
}
