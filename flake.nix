{
  description = "A flake for building an AI assistant";

  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.follows = "rust-overlay/flake-utils";
    nixpkgs.follows = "rust-overlay/nixpkgs";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
      code = pkgs.callPackage ./. { inherit pkgs system rust-overlay; };
    in rec {
      packages = {
        service = code.service;
        cli = code.cli;
        default = packages.service;
      };
      devShells = {
        default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-bin.stable.latest.default
            cmake
            gnumake
            extra-cmake-modules
          ];
          shellHook = ''
            export RUST_LOG=trace
          '';
        };
      };
    }
  );
}
