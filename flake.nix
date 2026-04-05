{
  description = "Rofi plugin development environment";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      eachSystem =
        fn:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
          "aarch64-linux"
        ] (system: fn (import nixpkgs { inherit system; }));
    in
    {
      packages = eachSystem (pkgs: {
        default = pkgs.callPackage ./package.nix { };
      });
      devShells = eachSystem (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            cairo
            cargo
            glib
            pango
            pkg-config
            rofi
            rust-analyzer
            rustc
            rustfmt
            sqlite
          ];
          shellHook = ''
            export ROFI_PLUGIN_PATH="$(pwd)/target/debug"
          '';
        };
      });
    };
}
