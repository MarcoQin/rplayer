fn main() {
    println!("cargo:rustc-flags=-l avformat  -l avcodec -l swscale -l avutil -l swresample -l z -l sdl2");
}