{
  description = "Rust project with crane and explicit targets";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };
  outputs = { self, nixpkgs, flake-utils, crane }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"

      ];
    in
    flake-utils.lib.eachSystem supportedSystems (system:
      let
        pkgs = import nixpkgs { inherit system; };
        craneLib = crane.mkLib pkgs;
        rustTarget = if system == "aarch64-darwin" then "aarch64-apple-darwin"
                     else if system == "x86_64-darwin" then "x86_64-apple-darwin"
                     else if system == "aarch64-linux" then "aarch64-unknown-linux-gnu"
                     else "x86_64-unknown-linux-gnu";
        commonArgs = {
          src = craneLib.cleanCargoSource ./.;
          buildInputs = with pkgs; [
            # Your dependencies
          ] ++ lib.optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
          ];
          CARGO_BUILD_TARGET = rustTarget;
          CARGO_TARGET = rustTarget;
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        myPackage = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
      in
      {
        packages.default = myPackage;
        hydraJobs = {
          package = myPackage;
          test = craneLib.cargoTest (commonArgs // {
            inherit cargoArtifacts;
          });
        };
        devShells.default = pkgs.mkShell {
          inputsFrom = [ myPackage ];
          packages = with pkgs; [
            cargo
            rustc
            rust-analyzer
            rustup
          ];
          CARGO_BUILD_TARGET = rustTarget;
          CARGO_TARGET = rustTarget;
        };
      }
    );
}
