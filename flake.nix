{
  description = "A commandline clock with nixie tubes";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    nix-systems.url = "github:nix-systems/default";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, ... }@inputs:
    let
      eachSystem = nixpkgs.lib.genAttrs (import inputs.nix-systems);
      mkPkgs = system:
        import nixpkgs {
          inherit system;
          overlays = [ inputs.rust-overlay.overlays.default ];
        };
    in {
      overlays.default =
        (final: prev: { nixie-clock = self.packages.${prev.system}.default; });
      packages = eachSystem (system:
        let pkgs = mkPkgs system;
        in {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "nixie-clock";
            version = "1.0.0";
            src = ./.;
            cargoHash = "sha256-tJVn1C+AOyhzJ+aviknor1m++CHveBWHIKNAmR9XWGM=";
            meta = {
              description =
                "A CLI clock that displays time in a Nixie tube style";
              longDescription = ''
                A unique command-line clock that displays the current time using Nixie tube-style digits.
                This charming design adds a vintage touch to your terminal while providing an accurate clock.
              '';

              homepage = "https://github.com/NewDawn0/nixie-clock";
              license = pkgs.lib.licenses.mit;
              maintainers = with pkgs.lib.maintainers; [ NewDawn0 ];
              platforms = pkgs.lib.platforms.all;
            };
          };
        });
    };
}
