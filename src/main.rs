mod config;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let matches = clap::App::new("fsync")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Sync any folders accross computers")
        .subcommand(clap::Command::new("cloud").about("Subcommands related to cloud storage"))
        .subcommand(clap::Command::new("local").about("about").alias("l"))
        .get_matches();

    if let Some(_sub_m) = matches.subcommand_matches("cloud") {
        handle_cloud_cmd().await?;
    }

    if let Some(_sub_m) = matches.subcommand_matches("local") {
        print!("local")
    }

    Ok(())
}

async fn handle_cloud_cmd() -> Result<(), rocket::Error> {
    open::that(config::GOOGLE_AUTH_URL).unwrap();
    rocket::build()
        .mount("/", routes![google_oath2_callback])
        .launch()
        .await?;
    Ok(())
}

#[get("/oauth2/google")]
fn google_oath2_callback(shutdown: rocket::Shutdown) -> &'static str {
    rocket::Shutdown::notify(shutdown);
    "Test"
}
