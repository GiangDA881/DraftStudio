# Source this (Git Bash) to build without MSVC Build Tools: Rust GNU toolchain + portable MinGW
# from `winget install BrechtSanders.WinLibs.POSIX.MSVCRT`. Point MINGW at another mingw64/bin to
# use a different MinGW. Machines with MSVC need none of this: plain `cargo build` works.
MINGW="${MINGW:-$(cygpath -u "$LOCALAPPDATA")/Microsoft/WinGet/Packages/BrechtSanders.WinLibs.POSIX.MSVCRT_Microsoft.Winget.Source_8wekyb3d8bbwe/mingw64/bin}"
export PATH="$HOME/.cargo/bin:$MINGW:$PATH"
export RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-gnu
export CARGO_BUILD_TARGET=x86_64-pc-windows-gnu
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER="$(cygpath -w "$MINGW/gcc.exe")"
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_AR="$(cygpath -w "$MINGW/ar.exe")"
