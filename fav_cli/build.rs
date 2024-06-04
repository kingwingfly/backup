use vergen::EmitBuilder;

fn main() {
    EmitBuilder::builder()
        .fail_on_error()
        .git_describe(false, true, None)
        .git_commit_timestamp()
        .emit()
        .ok();
    EmitBuilder::builder()
        .rustc_host_triple()
        .rustc_channel()
        .rustc_semver()
        .emit()
        .unwrap();
}
