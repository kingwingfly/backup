use protobuf::reflect::MessageDescriptor;
use protobuf_codegen::{Codegen, Customize, CustomizeCallback};
use rustc_version::{version_meta, Channel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set cfg flags depending on release channel
    let channel = match version_meta().unwrap().channel {
        Channel::Stable => "CHANNEL_STABLE",
        Channel::Beta => "CHANNEL_BETA",
        Channel::Nightly => "CHANNEL_NIGHTLY",
        Channel::Dev => "CHANNEL_DEV",
    };
    println!("cargo:rustc-cfg={}", channel);
    Codegen::new()
        .pure()
        .includes(["proto"])
        .inputs(["proto/bili.proto"])
        .out_dir("src/proto")
        .customize_callback(MyCustomizeCallback)
        .run()
        .ok(); // Just omit the err since crates.io build on a readonly system
    println!("cargo:return-if-changed=proto");
    println!("cargo:return-if-changed=build.rs");
    Ok(())
}

struct MyCustomizeCallback;

impl CustomizeCallback for MyCustomizeCallback {
    fn message(&self, message: &MessageDescriptor) -> Customize {
        let name = message.name();
        match name {
            "Upper" => Customize::default().before(DERIVE_ATTR),
            "BiliRes" | "BiliSet" => Customize::default().before(DERIVE_ATTR_STATUS),
            _ => Customize::default(),
        }
    }
}

const DERIVE_ATTR: &str = "#[derive(fav_core::attr::Attr)]";
const DERIVE_ATTR_STATUS: &str = "#[derive(fav_core::attr::Attr, fav_core::status::Status)]";
