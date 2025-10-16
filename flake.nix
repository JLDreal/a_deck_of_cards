{
  description = "test on Hydra";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "a_deck_of_cards";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          # Optional: Add build dependencies if needed
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          # Optional: Add runtime dependencies if needed
          buildInputs = with pkgs; [
            # openssl
          ];
        };

        # This is what Hydra will build and run tests for
        hydraJobs = {
          package = self.packages.${system}.default;
          test = self.packages.${system}.default.overrideAttrs (old: {
            doCheck = true;
          });
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
            pkg-config
          ];
        };
      }
    );
}
