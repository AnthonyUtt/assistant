{ pkgs, deps, ... }: {
  service = pkgs.rustPlatform.buildRustPackage {
    pname = "ai-assistant-service";
    version = "0.0.1";
    src = ./service;
    cargoBuildFlags = "";

    cargoLock = {
      lockFile = ./service/Cargo.lock;
    };

    nativeBuildInputs = deps;
  };
  cli = pkgs.rustPlatform.buildRustPackage {
    pname = "ai-assistant-cli";
    version = "0.0.1";
    src = ./cli;
    cargoBuildFlags = "";

    cargoLock = {
      lockFile = ./cli/Cargo.lock;
    };

    nativeBuildInputs = deps;
  };
}
