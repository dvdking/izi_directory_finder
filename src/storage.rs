use std::io::Write;
use std::path::PathBuf;
use std::{env, path};

pub struct DirInfo {
    pub path: path::PathBuf,
    pub count: u32,
}

pub struct Storage {
    storage_file_path: path::PathBuf,
    pub dir_info: Vec<DirInfo>,
}

impl Storage {
    pub fn new() -> Storage {
        let exe_path = env::current_exe().expect("Failed to get executing path");
        let exe_dir = exe_path.parent().expect("Failed to get executing dir");
        let storage_file_path = exe_dir.join(".mostuseddirs.txt");

        let dirs = Storage::load_storage(&storage_file_path);
        let dirs: Vec<&str> = dirs.split('\n').collect();

        let mut storage = Storage {
            storage_file_path,
            dir_info: Vec::new(),
        };

        storage.build_dir_info(dirs);
        storage
    }

    fn build_dir_info(&mut self, dirs: Vec<&str>) {
        self.dir_info = dirs
            .iter()
            .filter(|dir| !str::is_empty(dir))
            .map(|dir| {
                let dir: Vec<&str> = dir.split(' ').collect();
                DirInfo {
                    path: path::PathBuf::from(dir[0]),
                    count: dir[1].parse().unwrap(),
                }
            })
            .collect();

        let current_dir = env::current_dir().unwrap();
        self.increment_usage(&current_dir);
    }

    pub fn increment_usage(&mut self, directory: &PathBuf) {
        let mut found = false;
        for i in 0..self.dir_info.len() {
            if self.dir_info[i].path == *directory {
                self.dir_info[i].count += 1;
                found = true;
                break;
            }
        }

        if !found {
            self.dir_info.push(DirInfo {
                path: directory.to_path_buf(),
                count: 1,
            });
        }
    }

    fn load_storage(storage_file_path: &path::PathBuf) -> String {
        if !storage_file_path.exists() {
            std::fs::File::create(storage_file_path).expect("Unable to create storage");
        }

        std::fs::read_to_string(storage_file_path).expect("Unable to read storage")
    }

    pub fn save_storage(&self) {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.storage_file_path)
            .expect("Unable to open file");

        for dir in self.dir_info.iter() {
            let line = format!("{} {}\n", dir.path.to_str().unwrap(), dir.count);
            file.write_all(line.as_bytes()).unwrap();
        }
        file.flush().unwrap();
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}
