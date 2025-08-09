pub mod search;

// This file is the entry point for all route configurations.
// For now, we only have the search routes, but you can add more modules here.
pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    // Here, we configure all services from the search module.
    cfg.configure(search::config);
}
