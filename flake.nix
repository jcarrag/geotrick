{
  description = "geotrick";

  inputs.rustOverlay.url = "github:oxalica/rust-overlay";

  outputs = { self, unstable, rustOverlay }:
    let
      system = "x86_64-linux";

      pkgs = import unstable { inherit system; overlays = [ rustOverlay.overlays.default ]; };

      rust = pkgs.rust-bin.nightly.latest.default.override {
        extensions = [ "llvm-tools-preview" ];
        # TODO: is this the Cortex-M4 or Cortex-M4F? (https://docs.rust-embedded.org/book/start/qemu.html#cross-compiling)
        targets = [ "thumbv7em-none-eabihf" ];
      };
    in
    with pkgs; {
      devShell.${system} = mkShell {
        buildInputs = [
          rust
          gdb
          openocd

          # needed for `cargo install probe-run`
          pkg-config
          libudev-zero
        ];

        shellHook = ''
          export DEFMT_LOG=warn
        '';
      };
    };
}
