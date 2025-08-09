mod middleware;
mod routes;
mod services;

use actix_web::{
    // web::... for path, query, data extraction
    App,
    // ServiceRequest for middleware
    HttpServer,
    // Web application
    dev::ServiceRequest,
    // HTTP server
    web,
};

// actix_web_httpauth for authentication
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::middleware::HttpAuthentication;

// Utoipa for OpenAPI/Swagger documentation
use utoipa::OpenApi;
// Utoipa-swagger-ui for serving Swagger UI
use crate::middleware::auth;
use crate::services::search_service;
use utoipa_swagger_ui::{Config, SwaggerUi};

// Main function to run the Actix web server.
// #[actix_web::main] makes the main function asynchronous.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create a new instance of our API documentation, now co-located
    // with the search routes.
    let openapi = routes::search::ApiDoc::openapi();

    // Create a new search service
    let search_service = search_service::SearchService::new();

    // Start the HTTP server
    HttpServer::new(move || {
        // Create an authentication checker for our basic auth middleware
        let auth_checker =
            HttpAuthentication::basic(|req: ServiceRequest, auth: BasicAuth| async move {
                // Match on the result of the authentication check
                match auth::validate_basic_auth(&auth) {
                    // If validation is successful, pass the request on
                    Ok(()) => Ok(req),
                    // If validation fails, use the error returned by the function
                    // to construct the error response for the middleware.
                    Err(e) => Err((e, req)),
                }
            });

        let swagger_ui = SwaggerUi::new("/swagger-ui/{_:.*}")
            .url("/api-doc/openapi.json", openapi.clone());

        // Create the Actix web application instance
        App::new()
            // Add the Swagger UI to the application
            .service(swagger_ui)
            // Use the `configure` method to register all search routes from our new module
            .configure(routes::search::config)
            // Add our authentication middleware to the application.
            // This will protect all routes that follow.
            .wrap(auth_checker)
            .app_data(web::Data::new(search_service.clone()))
    })
    // Bind the server to a local address and port
    .bind(("127.0.0.1", 8080))?
    // Run the server and await its completion
    .run()
    .await
}
