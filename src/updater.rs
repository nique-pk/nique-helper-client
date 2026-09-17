// Nique build: the automatic updater is removed. New versions of the client ship
// inside the Nique hub installer; the client never checks or downloads anything.
use hbb_common::ResultType;
use std::path::PathBuf;

pub fn update_controlling_session_count(_count: usize) {}

#[allow(dead_code)]
pub fn start_auto_update() {}

#[allow(dead_code)]
pub fn manually_check_update() -> ResultType<()> {
    Ok(())
}

#[allow(dead_code)]
pub fn stop_auto_update() {}

pub fn get_download_file_from_url(_url: &str) -> Option<PathBuf> {
    None
}
