{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };
  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };
    in {
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          appstream
          glib
          gst_all_1.gstreamer
          gtk4
          libadwaita
          libxml2
          poppler
          alsa-lib
        ];

        nativeBuildInputs = with pkgs.buildPackages; [
          rust-bin.beta.latest.default

          appstream-glib # For appstream-util
          desktop-file-utils # For update-desktop-database
          dos2unix
          meson
          ninja
          pkg-config
          cmake
          python3 # For the postinstall script
          rustPlatform.bindgenHook
          rustPlatform.cargoSetupHook
          # cargo
          # rustc
          shared-mime-info # For update-mime-database
          wrapGAppsHook4
        ];
        packages = [];

        shellHook = ''
          export XDG_DATA_DIRS=$GSETTINGS_SCHEMAS_PATH:$XDG_DATA_DIRS
        '';
      };
    });
}
