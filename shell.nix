{ pkgs ? import <nixpkgs> {} }:

with pkgs;

let
  tex = (pkgs.texlive.combine {
    inherit (pkgs.texlive) scheme-small
      fvextra;
  });
  pythonWithPytest = pkgs.python310.buildEnv.override {
    extraLibs = with pkgs.python310Packages; [
      pip
      virtualenv
      pytest
    ];
    ignoreCollisions = true;
  };
in
mkShell.override { stdenv = pkgs.llvmPackages_18.stdenv; } {
  buildInputs = [
    pkgs.pandoc
    tex
    pkgs.cargo
    pkgs.rustc
    pkgs.rust-analyzer
    pkgs.gdb
    pkgs.libxml2
    pkgs.libxml2.dev
    pkgs.expat
    pkgs.expat.dev
    pythonWithPytest
    pkgs.nodePackages.pyright
    pkgs.gnumake

    pkgs.bashInteractive
  ];

  NIX_LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    pkgs.stdenv.cc.cc
    pkgs.zlib
  ];

  NIX_LD = pkgs.lib.fileContents "${pkgs.stdenv.cc}/nix-support/dynamic-linker";

  shellHook = ''
    export VENV_DIR="$PWD/.venv"
    if [ ! -d "$VENV_DIR" ]; then
      ${pythonWithPytest}/bin/python -m venv $VENV_DIR
      source $VENV_DIR/bin/activate
      pip install pip setuptools wheel
    else
      source $VENV_DIR/bin/activate
    fi

    export LD_LIBRARY_PATH=$NIX_LD_LIBRARY_PATH
    export PYTHONPATH="${pythonWithPytest}/lib/python3.10/site-packages:$PYTHONPATH"
    export REF5RSL="$PWD/lab2/refal/refal-5-framework/lib:$PWD/lab2/refal/refal-5-framework/lib/posix:$REF5RSL"

    if [ -f lab2/requirements.txt ]; then
      pip install -r lab2/requirements.txt
    fi

    python --version
  '';
}
