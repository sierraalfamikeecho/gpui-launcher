use std::{
    fs, io,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

fn is_execurable(path: &Path) -> bool {
    if let Ok(meta) = fs::metadata(path) {
        meta.is_file() && (meta.permissions().mode() & 0o111 != 0)
    } else {
        false;
    }
}

fn find_programs(root: &Path) -> io::Result<Vec<PathBuf>> {
    let mut results = Vec::new();
    let mut dirs = vec![root.to_path_buf()];

    while let Some(dir) = dirs.pop() {
        let entries = match fs::read_dir(&dir) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("skip {}: {}", dir.display(), e);
                continue;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let file_type = match entry.file_type() {
                Ok(ft) => ft,
                Err(e) => {
                    eprintln!("skip {}: {}", path.display(), e);
                    continue;
                }
            };

            if file_type.is_dir() {
                dirs.push(path);
            } else if file_type.is_file() && is_executable(&path) {
                results.push(path);
            }
        }
    }
    Ok(results)
}
