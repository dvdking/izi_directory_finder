use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::storage::DirInfo;

pub fn get_best_result<'a>(dir_info: &'a [DirInfo], search_str: &'a str) -> String {
    let matcher = SkimMatcherV2::default();

    let max_count = dir_info.iter().map(|a| a.count).max().unwrap_or(0) as f64;

    let best_result = dir_info
        .iter()
        .map(|dir_info| {
            (
                dir_info,
                calculate_item_score(&matcher, dir_info, search_str, max_count),
            )
        })
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();

    return best_result.0.path.to_str().unwrap_or_default().to_owned();
}

pub fn get_all_results<'a>(dir_info: &'a [DirInfo], search_str: &'a str) -> Vec<(&'a DirInfo, f64)> {
    let matcher = SkimMatcherV2::default();

    let max_count = dir_info.iter().map(|a| a.count).max().unwrap_or(0) as f64;

    dir_info
        .iter()
        .map(|dir_info| {
            (
                dir_info,
                calculate_item_score(&matcher, dir_info, search_str, max_count),
            )
        })
        .collect()
}

fn calculate_item_score(
    matcher: &SkimMatcherV2,
    dir_info: &DirInfo,
    search_str: &str,
    max_count: f64,
) -> f64 {
    let match_score = matcher
        .fuzzy_match(dir_info.path.to_str().unwrap(), search_str)
        .unwrap_or(0) as f64
        / 100.0;

    let mut final_score = 0.0;
    final_score += match_score;
    final_score += dir_info.count as f64 / max_count * match_score * 0.25;

    if dir_info
        .path
        .to_str()
        .unwrap()
        .to_lowercase()
        .ends_with(&search_str.to_lowercase())
    {
        final_score += match_score * 0.25;
    }
    final_score
}
