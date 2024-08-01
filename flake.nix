{
  description = "kubectl plugin to view Node provider IDs";

  inputs = {
    nixpkgs.url = "nixpkgs"; # Resolves to github:NixOS/nixpkgs
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };
  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        inherit (pkgs) lib;

        src = lib.cleanSourceWith { src = ./.; };
        nativeBuildInputs =
          with pkgs;
          [ pkg-config ]
          ++ lib.optionals (pkgs.stdenv.isDarwin) [
            libiconv
            darwin.apple_sdk.frameworks.Security
          ];
      in
      with pkgs;
      {
        devShell = pkgs.mkShell {
          nativeBuildInputs = nativeBuildInputs;
          buildInputs = with pkgs; [
            kubectl
            krew
            rust-bin.stable.latest.default
            rust-analyzer
          ];
        };
      }
    );
}
