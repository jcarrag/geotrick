{
  description = "geotrick";

  inputs.rustOverlay.url = "github:oxalica/rust-overlay";

  outputs = { self, unstable, rustOverlay }:
    let
      system = "x86_64-linux";

      pkgs = import unstable { inherit system; overlays = [ rustOverlay.overlay ]; };
      rust = pkgs.rust-bin.nightly."2021-07-08".default;

      # inherit (import-cargo.builders) importCargo;
    in
      with pkgs; {

        # packages.${system}.conduit = stdenv.mkDerivation {
        #   pname = "conduit";

        #   version = "0.1.0";

        #   # src = ./.;
        #   src = self;

        #   nativeBuildInputs = [
        #     pkgs.breakpointHook
        #     (importCargo { lockFile = ./Cargo.lock; inherit pkgs; }).cargoHome
        #     cargo
        #     rustc
        #   ];

        #   # sha256 = "";
        #   buildPhase = ''
        #     cargo build --release --offline
        #   '';

        #   installPhase = ''
        #     install -Dm775 ./target/release/conduit $out/bin/conduit
        #   '';
        # };

        # defaultPackage.${system} = self.packages.${system}.conduit;

        devShell.${system} = mkShell {
          buildInputs = [
            cargo-binutils
            cargo-generate
            gdb
            openocd
            openssl
            pkg-config
            qemu
            rust
            rustup

            libusb
          ];

          shellHook = ''
            export CONDUIT_CONFIG=./conduit.toml
          '';
        };
      };
}
