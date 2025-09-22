use poem::{get, post, handler, listener::TcpListener,  web::Path,Json, Route, Server};

#[handler]
fn get_website(Path(name): Path<String>) -> String {
    format!("website: {}", name)
}

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> 
Json<CreateWebsiteOutput> {

    let response = CreateWebsiteOutput {
        id: data.url
    };

    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app : Route = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
      .run(app)
      .await
}