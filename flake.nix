{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          pkg-config
          rustc
          cargo
        ];
        buildInputs = with pkgs; [
          dbus
          openssl
          glib
          gtk3          # ← gdk-3.0 はこれに含まれる
          libsoup_3     # ← libsoup-3.0
          webkitgtk_4_1 # ← javascriptcoregtk-4.1 もこれに含まれる
          librsvg
          pango
          gdk-pixbuf
          cairo
          harfbuzz
          at-spi2-atk
        ];
      };
    };
}
