{
  description = "A flake-friendler rewrite of nix command-not-found";

  inputs = {
    crane = {
      url = "github:ipetkov/crane";
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        craneLib = crane.mkLib pkgs;
        drv = craneLib.buildPackage { src = ./.; };
      in
      {
        checks.app-builds = drv;
        packages.default = drv;
        apps.default = flake-utils.lib.mkApp { inherit drv; };
        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks;
          nativeBuildInputs = with pkgs; [ cargo cargo-watch clippy rust-analyzer rustc rustfmt ];
        };
      });
}
