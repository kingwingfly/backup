use rustc_version::{version_meta, Channel};
use vergen::EmitBuilder;

fn main() {
    // Set cfg flags depending on release channel
    let channel = match version_meta().unwrap().channel {
        Channel::Stable => "CHANNEL_STABLE",
        Channel::Beta => "CHANNEL_BETA",
        Channel::Nightly => "CHANNEL_NIGHTLY",
        Channel::Dev => "CHANNEL_DEV",
    };
    println!("cargo:rustc-cfg={}", channel);

    EmitBuilder::builder()
        .git_describe(false, true, None)
        .rustc_host_triple()
        .rustc_channel()
        .rustc_semver()
        .emit()
        .unwrap();
}
