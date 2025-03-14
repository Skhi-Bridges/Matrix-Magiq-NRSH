{
  description = "Matrix-Magiq AO Process Management";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlay ];
        };

        # AO Process Packages
        aoPackages = {
          quantum-db-ao = pkgs.callPackage ./aos/quantum-db-ao {};
          user-db-ao = pkgs.callPackage ./aos/user-db-ao {};
          metrics-ao = pkgs.callPackage ./aos/metrics-ao {};
          tuning-ao = pkgs.callPackage ./aos/tuning-ao {};
        };

      in {
        packages = aoPackages;

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            lua5_4
            luajit
            rust-bin.stable.latest.default
            python310
            python310Packages.pip
            google-cloud-sdk
          ];
        };
      }
    );
}
