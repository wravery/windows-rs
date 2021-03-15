use super::*;

pub struct FileCache(pub Vec<File>);

impl FileCache {
    pub fn get() -> &'static Self {
        use std::{mem::MaybeUninit, sync::Once};
        static ONCE: Once = Once::new();
        static mut VALUE: MaybeUninit<FileCache> = MaybeUninit::uninit();

        ONCE.call_once(|| {
            // This is safe because `Once` provides thread-safe one-time initialization
            unsafe { VALUE = MaybeUninit::new(Self::from_iter(winmd_paths())) }
        });

        // This is safe because `call_once` has already been called.
        unsafe { &*VALUE.as_ptr() }
    }

    fn from_iter<I: IntoIterator<Item = std::path::PathBuf>>(files: I) -> Self {
        let mut files: Vec<File> = files.into_iter().map(File::new).collect();

        if !files.iter().any(|file| file.name.starts_with("Windows.")) {
            files.push(File::from_bytes(
                "Windows.Win32.winmd".to_string(),
                include_bytes!("../../default/Windows.Win32.winmd").to_vec(),
            ));
            files.push(File::from_bytes(
                "Windows.WinRT.winmd".to_string(),
                include_bytes!("../../default/Windows.WinRT.winmd").to_vec(),
            ));
        }

        Self(files)
    }
}

fn winmd_paths() -> Vec<std::path::PathBuf> {
    let mut windows_path = workspace_windows_dir();
    windows_path.push("winmd");

    let mut paths = vec![];

    if let Ok(files) = std::fs::read_dir(windows_path) {
        for file in files.filter_map(|file| file.ok()) {
            if let Ok(file_type) = file.file_type() {
                if file_type.is_file() {
                    let path = file.path();
                    if let Some("winmd") = path.extension().and_then(|extension| extension.to_str())
                    {
                        paths.push(file.path());
                    }
                }
            }
        }
    }

    paths
}
