use poem::{get, post, handler, listener::TcpListener,  web::Path,Json, Route, Server};

use store::Store;
use request_inputs::CreateWebsiteInput;
use request_outputs::CreateWebsiteOutput;
pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn get_website(Path(name): Path<String>) -> String {
    format!("website: {}", name)
}

#[handler]
fn create_website(Json(data): 
Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput>{
    let s = Store{};
    let id = s.create_website(); 
    let response = CreateWebsiteOutput {
        id
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