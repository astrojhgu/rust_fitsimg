# default.nix
with import <nixpkgs> {};
stdenv.mkDerivation {
    name = "mpi_rust"; # Probably put a more meaningful name here
    buildInputs = [
    autoconf automake
    autoconf
    libtool
    cfitsio
    pkgconfig
    cmake
    xorg.libX11
    xorg.libXrandr
    xorg.libXinerama
    xorg.libXcursor
    xorg.libXxf86vm
    xorg.libXi
    libGL
    ];
    hardeningDisable = [ "all" ];
    #buildInputs = [gcc-unwrapped gcc-unwrapped.out gcc-unwrapped.lib];
    LIBCLANG_PATH = llvmPackages.libclang+"/lib";
}
