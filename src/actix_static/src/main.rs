use actix_files as fs;
use actix_web::{web,HttpResponse, App, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	let dir_static = std::env::current_exe().unwrap().parent().unwrap().join("static");
	HttpServer::new(move || {
		App::new().route("/", web::get().to(index))
		.service(fs::Files::new("/static", &dir_static).show_files_listing())
	})
		.bind("0.0.0.0:8088")?
		.run()
		.await
}