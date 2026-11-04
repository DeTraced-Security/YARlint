use yarlint::config::{ConfigError, MapLoader, SourceLoader};

#[test]
fn an_empty_loader_discovers_nothing() {
    let loader = MapLoader::new();

    assert_eq!(loader.discover().unwrap(), None);
}

#[test]
fn single_makes_its_file_the_discovery_target() {
    let loader = MapLoader::single("yarlint.toml", "[globals]\n");

    let path = loader.discover().unwrap().expect("a path");
    assert_eq!(path, std::path::PathBuf::from("yarlint.toml"));

    let source = loader.load(&path).unwrap();
    assert_eq!(source.path, path);
    assert_eq!(source.text, "[globals]\n");
}

#[test]
fn with_adds_a_file_without_making_it_discoverable() {
    let loader = MapLoader::single("yarlint.toml", "a").with("shared/base.toml", "b");

    assert_eq!(
        loader.discover().unwrap(),
        Some(std::path::PathBuf::from("yarlint.toml"))
    );

    let extra = loader
        .load(std::path::Path::new("shared/base.toml"))
        .expect("the added file should load");
    assert_eq!(extra.text, "b");
}

#[test]
fn with_on_an_empty_loader_leaves_discovery_empty() {
    let loader = MapLoader::new().with("shared/base.toml", "b");

    assert_eq!(loader.discover().unwrap(), None);
    assert!(
        loader
            .load(std::path::Path::new("shared/base.toml"))
            .is_ok()
    );
}

#[test]
fn loading_a_missing_file_is_a_not_found_io_error() {
    let loader = MapLoader::new();

    let err = loader
        .load(std::path::Path::new("nope.toml"))
        .expect_err("missing file should fail");

    match err {
        ConfigError::Io { path, source } => {
            assert_eq!(path, std::path::PathBuf::from("nope.toml"));
            assert_eq!(source.kind(), std::io::ErrorKind::NotFound);
        }
        other => panic!("expected an io error, got {other:?}"),
    }
}
