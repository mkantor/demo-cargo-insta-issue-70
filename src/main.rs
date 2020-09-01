fn main() {}

#[test]
fn repro() {
    use insta::{assert_snapshot, Settings};
    use std::path::Path;
    use std::time::SystemTime;

    let non_constant_value = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();

    let mut settings = Settings::new();
    let snapshot_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("snapshots");
    settings.set_snapshot_path(snapshot_path);
    settings.bind(|| assert_snapshot!(non_constant_value));
}
