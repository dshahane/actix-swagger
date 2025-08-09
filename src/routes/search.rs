use actix_web::{HttpResponse, Responder, post, web};

// Serde for serialization/deserialization
use serde::{Deserialize, Serialize};

// Utoipa for OpenAPI/Swagger documentation
use utoipa::{OpenApi, ToSchema};

use crate::services::search_service::SearchService;

// Data structure for the search request payload
#[derive(Debug, Deserialize, ToSchema)]
pub struct SearchRequest {
    /// The search query string.
    pub query: String,
}

// Data structure for the search response
#[derive(Debug, Serialize, ToSchema)]
pub struct SearchResponse {
    /// A list of results from the search.
    pub results: Vec<String>,
}

// Define the OpenAPI documentation for this specific module
#[derive(OpenApi)]
#[openapi(
    paths(fedsearch_api, search_api),
    components(schemas(SearchRequest, SearchResponse)),
    security(
        ("basic_auth" = [])
    )
)]
pub struct ApiDoc;

// Define a federated search endpoint
#[utoipa::path(
    post,
    path = "/fedsearch",
    summary = "Performs a federated search across multiple sources.",
    description = "Searches for the given query on Google and Amazon and returns a combined list of results.",
    request_body = SearchRequest,
    responses(
        (status = 200, description = "Search successful", body = SearchResponse),
        (status = 401, description = "Unauthorized - Invalid credentials"),
    ),
    security(
        ("basic_auth" = [])
    )
)]
#[post("/fedsearch")]
async fn fedsearch_api(
    search_request: web::Json<SearchRequest>,
    search_service: web::Data<SearchService>,
) -> impl Responder {
    let results = search_service.federated_search(&search_request.query).await;

    let response = SearchResponse { results };
    HttpResponse::Ok().json(response)
}

// Define a simple search endpoint
#[utoipa::path(
    post,
    path = "/search",
    summary = "Performs a simple search.",
    description = "Performs a simple search that returns a single mock result.",
    request_body = SearchRequest,
    responses(
        (status = 200, description = "Search successful", body = SearchResponse),
        (status = 401, description = "Unauthorized - Invalid credentials"),
    ),
    security(
        ("basic_auth" = [])
    )
)]
#[post("/search")]
async fn search_api(
    search_request: web::Json<SearchRequest>,
    search_service: web::Data<SearchService>,
) -> impl Responder {
    let results = search_service.search(&search_request.query).await;

    let response = SearchResponse { results };
    HttpResponse::Ok().json(response)
}

// This function registers all the handlers for this module
pub fn config(configuration: &mut web::ServiceConfig) {
    configuration
        .service(fedsearch_api)
        .service(search_api);
}
