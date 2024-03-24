use std::{
    env, fs,
    path::{self, PathBuf},
};

pub mod match_finder;
pub mod storage;

fn main() {
    let args: Vec<String> = env::args().collect();
    let search_str = &args.get(1).expect("Missing path argument");

    let mut storage = storage::Storage::new();

    if *search_str == "-l" {
        let default_search = "".to_string();
        let search_str = &args.get(2).unwrap_or(&default_search);

        match_finder::get_all_results(&storage.dir_info, search_str)
            .iter()
            .for_each(|(dir, score)| {
                println!("{} {} {}", dir.path.to_str().unwrap(), dir.count, score);
            });

        return;
    }

    if *search_str == ".." {
        let current_dir = env::current_dir().unwrap();
        let parent_dir = current_dir.parent().unwrap();
        storage.increment_usage(&parent_dir.to_path_buf());
        storage.save_storage();
        println!("{}", parent_dir.to_str().unwrap());
        return;
    }

    if path::Path::new(search_str).exists() {
        let dir = fs::canonicalize(PathBuf::from(search_str)).unwrap();
        let dir: path::PathBuf = dir
            .iter() // iterate over path components
            .map(|x| x.to_str().unwrap().replace("\\\\?\\", ""))
            .collect();

        storage.increment_usage(&dir.to_path_buf());
        storage.save_storage();
        println!("{}", search_str);
        return;
    }

    let best_result = match_finder::get_best_result(&storage.dir_info, search_str);
    storage.increment_usage(&path::PathBuf::from(best_result.clone()).to_path_buf());
    storage.save_storage();

    println!("{}", best_result);
}
