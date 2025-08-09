
pub(crate) async fn perform_search(query: &str) -> Vec<String> {
    // Define a type alias for a boxed future to make the code cleaner.
    vec![
        String::from("This is tantivy result1"),
        String::from("This is tantivy result2"),
        String::from("This is tantivy result3"),
    ]
}
