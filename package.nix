{
  rustPlatform
}:

rustPlatform.buildRustPackage {
  pname = "clonck";
  version = "0.0.3";
  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;

  meta = {
    description = "clonck - a clock that isn't dumb and stupid";
    homepage = "https://github.com/jbwar22/clonck";
  };
}
