fn main() {
    let dst = cmake::build("consoleLib");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=ConsoleLib");

    #[cfg(unix)]
    {
        println!("cargo:rustc-link-lib=dylib=ncurses");
    }
}
