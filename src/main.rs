use actix_web::{middleware, web, App, HttpServer};
use actix_files::NamedFile;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Result};
use std::path::Path;

use std::env::set_var;

async fn serve_content(req: HttpRequest, config: web::Data<Config>) -> Result<HttpResponse> {
    let req_path = format!("/{}", req.match_info().query("route"));

    let status_code;

    let mut response_file = format!("{}/{}", &config.root, req_path);

    let p = Path::new(&response_file);

    if !p.exists() || p.is_dir() {
        status_code = StatusCode::OK;
        response_file = format!("{}/{}", &config.root, &config.index );
    } else {
        status_code = StatusCode::OK;
    }

    Ok(NamedFile::open(response_file)?
        .set_status_code(status_code)
        .prefer_utf8(true)
        .use_last_modified(true).into_response(&req)?)
}

#[derive(Debug, Clone)]
struct Config {
    root: String,
    index: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let matches = clap::App::new("server")
        .version("1.0")
        .arg(clap::Arg::with_name("listen")
            .short("l")
            .long("listen")
            .value_name("ADDRESS")
            .help("Specify host and port to listen on default '0.0.0.0:5000'")
            .env("BIND")
            .default_value("0.0.0.0:5000")
        )
        .arg(clap::Arg::with_name("root")
            .short("r")
            .long("root")
            .value_name("WEB_ROOT")
            .help("Specify the web root to serve defaults to '.'")
            .env("WEB_ROOT")
            .default_value(".")
        )
        .arg(clap::Arg::with_name("index")
            .short("i")
            .long("index")
            .value_name("INDEX")
            .help("Specify the fallback file to serve defaults to 'index.html'")
            .env("INDEX")
            .default_value("index.html")
        ).get_matches();

    let listen_addr = matches.value_of("listen").unwrap();

    let cfg = Config{
        index: matches.value_of("index").unwrap().to_owned(),
        root: matches.value_of("root").unwrap().to_owned()
    };

    // ASCII art banner always looks cool
    println!(
        "                            
         _   _                         
        | |_|_|___ ___ ___ ___ _ _ ___ 
        | . | |   |_ -| -_|  _| | | -_|
        |___|_|_|_|___|___|_|  \\_/|___| v0.1.0
    "
    );

    // print out `host` and `port` of the server
    println!(
        "\nYour server is up and running at http://{}/\n",
        listen_addr
    );

    HttpServer::new(move || {
        //`.show_files_listing()` mode is set if directory listing is enabled in config
        
            App::new()
            .data(cfg.clone())
            // enable the logger middlware
            .wrap(middleware::Logger::default())
            .route("/{route:.*}", web::get().to(serve_content))
    })
    .bind(listen_addr)?
    .run()
    .await
}
