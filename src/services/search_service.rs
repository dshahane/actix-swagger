use crate::services::federated_search;
use crate::services::tantivy_search;

// SearchService struct to hold application state, if needed.
// This is now responsible for coordinating the federated search.
#[derive(Clone)]
pub struct SearchService;

impl SearchService {
    pub fn new() -> Self {
        Self
    }

    // A method to perform the federated search
    pub async fn federated_search(&self, query: &str) -> Vec<String> {
        federated_search::perform_federated_search(query).await
    }

    pub async fn search(&self, query: &str) -> Vec<String> {
        tantivy_search::perform_search(query).await
    }
}