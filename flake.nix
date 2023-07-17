{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        devToolchain = fenix.packages.${system}.stable;
      in
      with pkgs;
      {
        devShell = mkShell {
          nativeBuildInputs = with pkgs; [
            (devToolchain.withComponents [
              "cargo"
              "clippy"
              "rust-src"
              "rustc"
              "rustfmt"
            ])
            fenix.packages.${system}.rust-analyzer
          ] ++ (
            lib.optionals pkgs.stdenv.isDarwin [
              pkgs.darwin.apple_sdk.frameworks.CoreServices
              pkgs.libiconv
            ]
          );

          # buildInputs = with pkgs; [
          #   python37Packages.rethinkdb
          # ];
        };
      }
    );
}
