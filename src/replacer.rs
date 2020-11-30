use ruplacer::{Settings, DirectoryPatcher, query};
use std::path::{Path};

pub async fn run_ruplacer(foo: &str, bar: &str, data_path: &str) -> DirectoryPatcher {
    let mut settings = Settings::default();
    settings.dry_run = false;
    let path = Path::new(data_path);
    let mut directory_patcher = DirectoryPatcher::new(path.to_path_buf(), settings);
    directory_patcher
        .patch(&query::substring(foo, bar))
        .expect("ruplacer failed");
    directory_patcher
}