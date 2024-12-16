with import <nixpkgs> {};
mkShell {
    buildInputs = [ rustup gcc bacon openssl pkg-config ];
    LD_LIBRARY_PATH = lib.makeLibraryPath [ openssl ];
}
