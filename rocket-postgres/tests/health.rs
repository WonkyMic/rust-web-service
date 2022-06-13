#[path = "common/utils.rs"]
mod utils;

use rocket::http::{ContentType, Status};

#[test]
fn is_healthy() {
    let client = utils::test_client();
    let response = &mut client
        .get("/wonky/health")
        .dispatch();
    
    assert_eq!(response.status(), Status::Ok);
}