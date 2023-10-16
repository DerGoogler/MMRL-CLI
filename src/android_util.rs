use mnt::get_mount;
use std::path::Path;

#[warn(dead_code)]
pub(crate) fn list_mount(target: &Path) -> bool {
    match get_mount(&target) {
        Ok(list) => match list {
            Some(mount) => true,
            None => false,
        },
        Err(e) => false,
    }
}
