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
        "x86_64-darwin"
        "aarch64-darwin"
      ];
    in
    flake-utils.lib.eachSystem supportedSystems (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;


        commonArgs = {
          src = craneLib.cleanCargoSource ./.;

          buildInputs = with pkgs; [
            # Your dependencies
          ] ++ lib.optionals pkgs.stdenv.isDarwin [
                      # Additional darwin specific inputs can be set here
                      pkgs.libiconv
                    ];

          # Explicitly set the target
          CARGO_BUILD_TARGET = pkgs.stdenv.hostPlatform.config;
          CARGO_TARGET = pkgs.stdenv.hostPlatform.config;
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
          ];

          # Set environment variables in the shell
          CARGO_BUILD_TARGET = pkgs.stdenv.hostPlatform.config;
        };
      }
    );
}
