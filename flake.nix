{
  description = "A very basic flake for wkeys";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let 
      system = builtins.currentSystem;
      pkgs = import nixpkgs {};
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
      pkgs.${system}.default = wkeys_build;
    };
}
