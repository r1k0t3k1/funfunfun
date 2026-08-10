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
          nodejs_latest
          playwright-driver.browsers
        ];
        
        shellHook = ''
	  export PATH=~/.cargo/bin:$PATH
          export CHROMIUM_BIN=${pkgs.chromium}/bin/chromium
	  export PLAYWRIGHT_BROWSERS_PATH=${pkgs.playwright-driver.browsers}
	  export PLAYWRIGHT_SKIP_VALIDATE_HOST_REQUIREMENTS=true;
        '';

        buildInputs = with pkgs; [
          chromium
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
