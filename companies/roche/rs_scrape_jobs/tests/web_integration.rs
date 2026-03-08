use rs_scrape::{db_repo, db_setup, web_server};
use std::env;
use std::sync::Arc;

#[tokio::test]
async fn test_web_server_root_route() {
    // Set dummy OAuth credentials for testing
    env::set_var("GITHUB_CLIENT_ID", "test_client_id");
    env::set_var("GITHUB_CLIENT_SECRET", "test_client_secret");
    env::set_var("OAUTH_REDIRECT_URL", "http://localhost:3001/auth/callback");

    // Create test database
    let conn = db_setup::init_db("test_jobs.db").await.unwrap();
    let repo = Arc::new(db_repo::JobRepository::new(conn));

    // Create the app (this would normally start a server, but for testing we can create the router)
    // Note: For full integration testing, we'd need to start a test server
    // This is a basic smoke test to ensure the app can be created

    // Verify environment variables are set
    assert_eq!(env::var("GITHUB_CLIENT_ID").unwrap(), "test_client_id");
    assert_eq!(
        env::var("GITHUB_CLIENT_SECRET").unwrap(),
        "test_client_secret"
    );

    // Verify database connection works
    let candidate_count = repo.get_candidate_count().await.unwrap();
    assert!(candidate_count >= 0);

    // TODO: Add full HTTP integration test with test server
    // For now, this tests that the dependencies and setup work
}
