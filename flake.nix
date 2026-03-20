{
  description = "Voting App";

  nixConfig = {
    extra-substituters = [
      "https://scottylabs.cachix.org"
      "https://nix-community.cachix.org"
    ];
    extra-trusted-public-keys = [
      "scottylabs.cachix.org-1:hajjEX5SLi/Y7yYloiXTt2IOr3towcTGRhMh1vu6Tjg="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };

  inputs = {
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    devenv.url = "github:cachix/devenv";
    bun2nix = {
      url = "github:nix-community/bun2nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, devenv, bun2nix, ... }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = system: import nixpkgs {
        inherit system;
        overlays = [ bun2nix.overlays.default ];
      };
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = pkgsFor system;
        in
        {
          devenv = devenv.packages.${system}.devenv;
        }
        // (nixpkgs.lib.optionalAttrs (system == "x86_64-linux") (
          let
            votingAppFrontend = pkgs.stdenv.mkDerivation {
              pname = "voting-app-frontend";
              version = (builtins.fromJSON (builtins.readFile ./frontend/package.json)).version;
              src = ./frontend;

              nativeBuildInputs = [
                pkgs.bun2nix.hook
              ];

              bunDeps = pkgs.bun2nix.fetchBunDeps {
                bunNix = ./frontend/bun.nix;
              };

              buildPhase = ''
                bun run build
              '';

              installPhase = ''
                mkdir -p $out
                cp -r dist/* $out/
              '';
            };

            cargoNix = pkgs.callPackage ./backend/Cargo.nix { };

            votingAppBackend = cargoNix.workspaceMembers."backend".build;

          in
          {
            inherit votingAppFrontend votingAppBackend;
            default = votingAppBackend;
          }
        ))
      );
    };
}