{
  description = "A very basic flake for wkeys";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = { self, nixpkgs, flake-parts}@inputs: 
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      perSystem = {pkgs, system, lib,...}:
        let 
          wkeys_build = pkgs.rustPlatform.buildRustPackage {
            pname = "wkeys";
            version = "v0.1.0";

            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            nativeBuildInputs = with pkgs; [pkg-config cargo];
            buildInputs = with pkgs; [gtk4-layer-shell gtkmm4 libxkbcommon];
          };
        in 
          {
          packages.default = wkeys_build;
        };
    };
}
