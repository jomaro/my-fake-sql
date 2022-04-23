use super::{QueryExecutor, QueryResult};
use itertools::Itertools;
use std::io::Result;

pub struct QuerySanitizer<T>(T);

impl<T> QuerySanitizer<T> {
    pub fn new(executor: T) -> Self {
        Self(executor)
    }
}

impl<T> QueryExecutor for QuerySanitizer<T>
where
    T: QueryExecutor,
{
    fn query(&mut self, query: &str) -> Result<QueryResult> {
        let mut query: String = String::from(query.trim());
        if query.starts_with('/') {
            query = remove_comments_from_the_start(query);
        }
        self.0.query(&query)
    }
}

fn remove_comments_from_the_start(query: String) -> String {
    // There are some very crazy allocations happening here. We can probably clean it up later
    query
        .chars()
        .tuple_windows()
        .skip_while(|(prev, next)| !(*prev == '*' && *next == '/'))
        .skip(1)
        .map(|(_, next)| next)
        .collect::<String>()
        .trim()
        .to_string()
}