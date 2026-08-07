let pkgs = import <nixpkgs> { };
in pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    wrapGAppsHook4
    cargo 
    cargo-tauri
    nodejs
    rustc
  ];

  buildInputs = with pkgs; [
    librsvg
    webkitgtk_4_1
  ];

  shellHook = ''
    export XDG_DATA_DIRS="$GSETTINGS_SCHEMAS_PATH"
  '';
}
