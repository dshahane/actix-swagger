use actix_web::{
    // Macro to define HTTP GET endpoint
    dev::ServiceRequest,
    // Web application
    post,
    // HTTP server
    web,
    // Responder trait for handlers
    App,
    // web::... for path, query, data extraction
    HttpResponse,
    // HTTP response
    HttpServer
    // actix_web::Error
    ,
    // ServiceRequest for middleware
    Responder,
};

// actix_web_httpauth for authentication
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::middleware::HttpAuthentication;

// Serde for serialization/deserialization
use serde::{Deserialize, Serialize};

// For defining API documentation and security schemas
// Utoipa for OpenAPI/Swagger documentation
use utoipa::{
    // Macro to define API documentation
    OpenApi,
    // Macro to define schema for data structures
    ToSchema,
};
// Utoipa-swagger-ui for serving Swagger UI
use utoipa_swagger_ui::SwaggerUi;

// The following modules are for our custom application logic
// Search service implementation
mod search_service;
// Authentication logic
mod auth;

// Main function to run the Actix web server.
// #[actix_web::main] makes the main function asynchronous.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Define the OpenAPI documentation using the Utoipa macro
    #[derive(OpenApi)]
    #[openapi(
        // Define paths (endpoints) in the API
        paths(search_api),
        // Define data schemas used by the API
        components(schemas(SearchRequest, SearchResponse)),
        // Define security schemes for the API
        security(
            // Define the name of the security scheme and the scopes it applies to
            ("basic_auth" = [])
        )
    )]
    struct ApiDoc;

    // Create a new instance of our API documentation
    let openapi = ApiDoc::openapi();

    // Create a new search service
    let search_service = search_service::SearchService::new();

    // Start the HTTP server
    HttpServer::new(move || {
        // Create an authentication checker for our basic auth middleware
        // The closure now correctly returns the ServiceRequest on success or a tuple on failure.
        let auth_checker = HttpAuthentication::basic(|req: ServiceRequest, auth: BasicAuth| async move {
            match auth::validate_basic_auth(&auth) {
                // If validation is successful, pass the request on
                Ok(()) => Ok(req),
                // If validation fails, use the error returned by the function
                // to construct the error response for the middleware.
                Err(e) => Err((e, req)),
            }
        });

        // Create the Actix web application instance
        App::new()
            // Add the Swagger UI to the application
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    // Set the path to the OpenAPI JSON file
                    .url("/api-doc/openapi.json", openapi.clone())
            )
            // Register our search API endpoint
            .service(search_api)
            // Add our authentication middleware to the application.
            // This will protect all routes that follow.
            .wrap(auth_checker)
    })
        // Bind the server to a local address and port
        .bind(("127.0.0.1", 8080))?
        // Run the server and await its completion
        .run()
        .await
}

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

// Define a web search endpoint
#[utoipa::path(
    // HTTP method and path
    post,
    path = "/search",
    // Summary of the endpoint
    summary = "Performs a web search.",
    // Description of the endpoint
    description = "Searches the web for the given query and returns a list of results.",
    // Request body definition
    request_body = SearchRequest,
    // Response definitions
    responses(
        (status = 200, description = "Search successful", body = SearchResponse),
        (status = 401, description = "Unauthorized - Invalid credentials"),
    ),
    // Define the security requirement for this path
    security(
        ("basic_auth" = [])
    )
)]
#[post("/search")]
async fn search_api(search_request: web::Json<SearchRequest>) -> impl Responder {
    // For this example, we'll just return a mock response.
    // In a real application, you would integrate a real search service here.
    let response = SearchResponse {
        results: vec![
            format!("Result 1 for: {}", search_request.query),
            format!("Result 2 for: {}", search_request.query),
            format!("Result 3 for: {}", search_request.query),
        ],
    };
    HttpResponse::Ok().json(response)
}