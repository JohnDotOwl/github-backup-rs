use std::collections::HashSet;

use regex::Regex;

use crate::api::types::Repository;

#[derive(Debug, Clone, Default)]
pub struct RepositoryFilters {
    pub include_name: Option<Regex>,
    pub exclude_name: Option<Regex>,
    pub include_languages: HashSet<String>,
    pub exclude_archived: bool,
}

pub fn should_include_repository(repo: &Repository, filters: &RepositoryFilters) -> bool {
    if filters.exclude_archived && repo.archived {
        return false;
    }

    if let Some(pattern) = &filters.include_name {
        if !pattern.is_match(&repo.full_name) {
            return false;
        }
    }

    if let Some(pattern) = &filters.exclude_name {
        if pattern.is_match(&repo.full_name) {
            return false;
        }
    }

    if !filters.include_languages.is_empty() {
        let language = repo.language.as_deref().unwrap_or("");
        if !filters
            .include_languages
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case(language))
        {
            return false;
        }
    }

    true
}
