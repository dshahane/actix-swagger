use futures::future::{join_all, BoxFuture};
use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

// This is a placeholder for a real search implementation.
// In a real application, you would use a client library for Google Search.
async fn search_google(query: String) -> Vec<String> {
    // Simulate a network request
    sleep(Duration::from_millis(100)).await;
    println!("Searching Google for: {}", query);
    vec![
        format!("Google Result 1 for '{}'", query),
        format!("Google Result 2 for '{}'", query),
    ]
}

// This is a placeholder for a real Amazon search implementation.
// In a real application, you would use a client library for the Amazon Product Advertising API.
async fn search_amazon(query: String) -> Vec<String> {
    // Simulate a network request
    sleep(Duration::from_millis(150)).await;
    println!("Searching Amazon for: {}", query);
    vec![
        format!("Amazon Result 1 for '{}'", query),
        format!("Amazon Result 2 for '{}'", query),
    ]
}

// The main function to perform the federated search.
// It takes a query and runs both search functions concurrently.
pub async fn perform_federated_search(query: &str) -> Vec<String> {
    // Define a type alias for a boxed future to make the code cleaner.
    type SearchFuture = BoxFuture<'static, Vec<String>>;

    // Create a vector of boxed futures for each search operation.
    // We clone the query for each task to satisfy the 'static lifetime.
    let search_tasks: Vec<SearchFuture> = vec![
        Box::pin(search_google(query.to_owned())),
        Box::pin(search_amazon(query.to_owned())),
    ];

    // Wait for all search tasks to complete.
    let results: Vec<Vec<String>> = join_all(search_tasks).await;

    // Flatten the vector of vectors into a single vector.
    results.into_iter().flatten().collect()
}
