{
  description = "geotrick";

  inputs.rustOverlay.url = "github:oxalica/rust-overlay";

  inputs.mach-nix.url = "github:DavHau/mach-nix";

  outputs = { self, unstable, rustOverlay, mach-nix }:
    let
      system = "x86_64-linux";

      pkgs = import unstable { inherit system; overlays = [ rustOverlay.overlays.default ]; };

      # rust = pkgs.rust-bin.nightly."2021-07-08".default.override {
      rust = pkgs.rust-bin.nightly.latest.default.override {
        # TODO: is this the Cortex-M4 or Cortex-M4F? (https://docs.rust-embedded.org/book/start/qemu.html#cross-compiling)
        targets = [ "thumbv7em-none-eabihf" ];
      };

      adafruit-nrfutil = (import mach-nix { inherit pkgs; _default = "nixpkgs,wheel,sdist"; }).buildPythonPackage {
        src = pkgs.fetchFromGitHub {
          owner = "adafruit";
          repo = "Adafruit_nRF52_nrfutil";
          rev = "a4a666130e7da155ff9e20c556fbb803ef8d8227";
          sha256 = "mHHKOQE9AGBX8RAyaPOy+JS3fTs98+AFdq9qsVy7go4=";
        };
      };

      python-packages = pkgs.python3.withPackages (ppkgs: with ppkgs; [ adafruit-nrfutil ]);
    in
    with pkgs; {
      devShell.${system} = mkShell {
        buildInputs = [
          arduino
          arduino-core
          arduino-cli

          rls
          rust

          # cargo-embed
          pkgs.cargo-embed
          # pkg-config
          # libudev-zero
          # libusb1
          # libftdi1

          python-packages
        ];
      };
    };
}
