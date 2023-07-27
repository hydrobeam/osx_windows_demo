fn main() {
    let mut builder = cc::Build::new();
    builder.compiler("clang");
    builder.file("objc/streamcapturekit.m");

    // for flag in std::env::var("DEP_OBJC_0_3_CC_ARGS").unwrap().split(' ') {
    //     builder.flag(flag);
    // }

    builder.compile("comp/outer.a");
}
