// Tonic gRPC server should be ready first.
// I expect tests similar to what used in this post.
// https://www.steadylearner.com/blog/read/How-to-use-CORS-and-OPTIONS-HTTP-request-with-Rust-Rocket
// https://docs.rs/warp/0.1.20/warp/test/struct.RequestBuilder.html
#[tokio::test]
async fn get_user() {
    let _ = pretty_env_logger::init();

    let get_user = path!("api" / "user" / "v1")
        .and(warp::path::param::<String>())
        .and_then(get_hashed_user_info);

    let res = warp::test::request().path("/api/user/v1/steadylearner") // 1. Define path with datas
        .reply(&get_user).await; // 2. Use the exact payload you want to test and reply can be target, responder, reply_with.

    assert_eq!(res.status(), 200, "Should return 200 OK.");
    println!("{:#?}", res.body());
}