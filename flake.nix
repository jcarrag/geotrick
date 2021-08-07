{
  description = "geotrick";

  inputs.rustOverlay.url = "github:oxalica/rust-overlay";

  outputs = { self, unstable, rustOverlay }:
    let
      system = "x86_64-linux";

      pkgs = import unstable { inherit system; overlays = [ rustOverlay.overlay ]; };

      rust = pkgs.rust-bin.nightly."2021-07-08".default.override {
        targets = [ "thumbv6m-none-eabi" ];
      };
    in
      with pkgs; {
        devShell.${system} = mkShell {
          buildInputs = [
            rls
            rust
          ];
        };
      };
}
