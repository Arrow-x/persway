{ stdenv, rustPlatform, self }:

rustPlatform.buildRustPackage {
  pname = "persway";
  version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;
  src = self;
  cargoSha256 = "sha256-YFTE4iteUBRtcyeCHqDyaxsWN4oESpfGkcljPBMT2tQ=";
  doCheck = false;
  meta = {
    license = stdenv.lib.licenses.mit;
    maintainers = [
      {
        email = "john@insane.se";
        github = "johnae";
        name = "John Axel Eriksson";
      }
    ];
  };
}