{
  description = "A very basic flake for wkeys, by Yadobler";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs:

    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      flake.overlays = rec {
        default = wkeys;
        wkeys = final: _: {
          inherit (inputs.self.packages.${final.stdenv.hostPlatform.system}) wkeys;
        };
      };

      perSystem =
        {
          pkgs,
          ...
        }:

        {
          packages = rec {
            default = wkeys;
            wkeys = pkgs.callPackage (
              {
                lib,
                rustPlatform,
                pkg-config,
                libcosmicAppHook,
                gtk4-layer-shell,
                gtkmm4,
                libxkbcommon,
              }:

              let
                cargoToml = lib.importTOML ./wkeys/Cargo.toml;
              in

              rustPlatform.buildRustPackage {
                pname = cargoToml.package.name;
                inherit (cargoToml.package) version;

                src = ./.;
                # Update on change to Cargo.lock
                cargoHash = "sha256-deAiMKHkPKDWM57IL1xOvcqEzm0cLC6SktbKCDXZPZE=";

                nativeBuildInputs = [
                  pkg-config
                  libcosmicAppHook
                ];

                buildInputs = [
                  gtk4-layer-shell
                  gtkmm4
                  libxkbcommon
                ];

                preInstall = ''
                  mkdir -p $out/share/{applications,cosmic/net.pithos.applet.wkeys}

                  install -m644 $src/cosmic-applet/assets/wkeys-applet.desktop \
                    $out/share/applications/net.pithos.applet.wkeys.desktop
                '';

                meta = {
                  homepage = "https://github.com/ptazithos/wkeys";
                  description = "On-screen keyboard for the COSMIC Desktop Environment";
                  license = lib.licenses.mit;
                  maintainers = with lib.maintainers; [ nanoyaki ];
                  platforms = lib.platforms.linux;
                  mainProgram = "wkeys";
                };
              }
            ) { };
          };
        };
    };
}
