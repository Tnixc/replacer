{
  lib,
  rustPlatform,
  version ? "latest",
  ...
}:
rustPlatform.buildRustPackage {
  pname = "replacer-cli";
  inherit version;

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  meta = {
    description = "A flexible cli to replace strings in files or a directory";
    homepage = "https://github.com/tnixc/replacer";
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [tnixc];
    mainProgram = "replacer";
  };
}
