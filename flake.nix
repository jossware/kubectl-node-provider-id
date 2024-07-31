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

        krew-release-bot = pkgs.buildGoModule rec {
          pname = "krew-release-bot";
          version = "0.0.46";

          src = pkgs.fetchFromGitHub {
            rev = "v${version}";
            owner = "rajatjindal";
            repo = "krew-release-bot";
            sha256 = "sha256-73r4kT+J5DeAx0g5RcLBIJwBrXfmRwjRXFoEhmEVu/M=";
          };

          vendorHash = null;

          nativeBuildInputs = [ pkgs.installShellFiles ];

          subPackages = [ "cmd/action" ];

          CGO_ENABLED = 0;

          ldflags = [
            "-s"
            "-w"
          ];

          # Rename the output binary
          postInstall = ''
            mv $out/bin/action $out/bin/krew-release-bot
          '';

          doCheck = false;
        };

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
            krew-release-bot
            rust-bin.stable.latest.default
            rust-analyzer
          ];
        };
      }
    );
}
