# Readme  

## Env
D:\gitpath\pro\owneditor\src\part1>cc -v
Using built-in specs.
COLLECT_GCC=cc
COLLECT_LTO_WRAPPER=C:/msys64/mingw64/bin/../lib/gcc/x86_64-w64-mingw32/8.2.1/lto-wrapper.exe 
Target: x86_64-w64-mingw32
Configured with: ../gcc-8-20181214/configure --prefix=/mingw64 --with-local-prefix=/mingw64/local --build=x86_64-w64-mingw32 --host=x86_64-w64-mingw32 --target=x86_64-w64-mingw32 --with-native-system-header-dir=/mingw64/x86_64-w64-mingw32/include --libexecdir=/mingw64/lib --enable-bootstrap --with-arch=x86-64 --with-tune=generic --enable-languages=ada,c,lto,c++,objc,obj-c++,fortran --enable-shared --enable-static --enable-libatomic --enable-threads=posix --enable-graphite --enable-fully-dynamic-string --enable-libstdcxx-filesystem-ts=yes --enable-libstdcxx-time=yes --disable-libstdcxx-pch --disable-libstdcxx-debug --disable-isl-version-check --enable-lto --enable-libgomp --disable-multilib --enable-checking=release --disable-rpath --disable-win32-registry --disable-nls --disable-werror --disable-symvers --with-libiconv --with-system-zlib --with-gmp=/mingw64 --with-mpfr=/mingw64 --with-mpc=/mingw64 --with-isl=/mingw64 --with-pkgversion='Rev1, Built by MSYS2 project' --with-bugurl=https://sourceforge.net/projects/msys2 --with-gnu-as --with-gnu-ldThread model: posixgcc version 8.2.1 20181214 (Rev1, Built by MSYS2 project)

## Error
D:\gitpath\pro\owneditor\src\part1>cc echo.c
echo.c:1:10: fatal error: termios.h: No such file or directory
 #include <termios.h>
          ^~~~~~~~~~~
compilation terminated.

- At ubuntu 
  cc echo.c 
  It is ok, so windows7 env is not created complately.
