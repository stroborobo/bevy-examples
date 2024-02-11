#{
#  inputs = {
#    naersk.url = "github:nix-community/naersk/master";
#    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
#    utils.url = "github:numtide/flake-utils";
#  };
#
#  outputs = { self, nixpkgs, utils, naersk }:
#    utils.lib.eachDefaultSystem (system:
#      let
#        pkgs = import nixpkgs { inherit system; };
#        naersk-lib = pkgs.callPackage naersk { };
#      in
#      {
#        defaultPackage = naersk-lib.buildPackage ./.;
#        devShell = with pkgs; mkShell {
#          buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy ];
#          RUST_SRC_PATH = rustPlatform.rustLibSrc;
#        };
#      });
#}

# source: https://git.saragerretsen.nl/Hertog/RustThingy/src/branch/main/flake.nix
{
  description = "Rust dev shell for Hertog's bevy project (YAY!)";

  # Flake inputs
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk/master";
    nixgl.url = "github:guibou/nixGL"; # Allows you to run OpenGL and or Vulkan applications in a nix shell
  };

  # Flake outputs
  outputs = { nixpkgs, flake-utils, naersk, nixgl, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let

        # Overlays enable you to customize the Nixpkgs attribute set
        overlays = [
          nixgl.overlay
        ];
        pkgs = import nixpkgs { inherit overlays system; };

        naersk-lib = pkgs.callPackage naersk { };

      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        # Development environment output
        devShell = with pkgs; mkShell {
            # The Nix packages provided in the environment
            packages = (with pkgs; [
  #            # Fluff
  #            cargo-mommy
  #            onefetch
  #            # Bevy
  #            pkg-config
  #            #alsa-lib
  #            vulkan-tools
  #            vulkan-headers
  #            vulkan-loader
  #            #vulkan-validation-layers
  #            #udev
  #            clang
  #            lld
  ##             # If using an intel GPU
  ##             pkgs.nixgl.nixVulkanIntel
  ##             # If on x11
  ##             xorg.libX11
  ##             xorg.libX11
  ##             xorg.libXcursor
  ##             xorg.libXi
  ##             xorg.libXrandr
  ##             # If on wayland
  ##             libxkbcommon
  ##             wayland
  #            # Rust
  #            rustup
  #            rustToolchain
            ]) ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [ libiconv ];
            nativeBuildInputs = [
              rustPlatform.bindgenHook
              cargo
              rustc
              rustfmt
              pre-commit
              rustPackages.clippy
            ];
            buildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin (with darwin.apple_sdk.frameworks; [
              AppKit
              AudioUnit
              CoreAudio
              CoreAudioKit
            ]);
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
            shellHook =
              ''
                  export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:${pkgs.stdenv.cc.cc.lib}/lib:$LD_LIBRARY_PATH:/System/Library/Frameworks/;
              '';
  #          shellHook = ''
  #            # Required
  #            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
  #              pkgs.alsaLib
  #              pkgs.udev
  #              pkgs.vulkan-loader
  #            ]}"
  #            # Aliases and other fluff/ease of use
  #            alias runIntel="nixVulkanIntel cargo run"
  #            alias runMommyIntel="nixVulkanIntel cargo mommy run"
  #            onefetch
  #            echo "Welcome to nix-hell uh nix-shell!"
  #          '';
          };
      });
}
