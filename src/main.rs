#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate rocket;

mod config;

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
        println!("Cloud command concluded.");
    }

    if let Some(_sub_m) = matches.subcommand_matches("local") {
        print!("local")
    }

    Ok(())
}

async fn handle_cloud_cmd() -> Result<(), rocket::Error> {
    println!("Taking you to the Google Auth page...");

    let url = format!(
        "{}?client_id={}&redirect_uri={}&scope={}&response_type=code&access_type=offline",
        config::GOOGLE_AUTH_BASE_URI,
        config::GOOGLE_OAUTH_CLIENT_ID,
        config::GOOGLE_REDIRECT_URI,
        config::GOOGLE_SCOPE,
    );

    open::that(url).unwrap();

    rocket::build()
        .mount("/", routes![google_oath2_callback])
        .launch()
        .await?;

    let code = std::fs::read_to_string(config::AUTH_CODE_PATH).unwrap();
    println!("Code: {}", code);

    Ok(())
}

#[get("/oauth2/google?<code>")]
fn google_oath2_callback(shutdown: rocket::Shutdown, code: Option<String>) -> String {
    let result: String;

    match code {
        Some(val) => {
            std::fs::write(config::AUTH_CODE_PATH, val).unwrap();
            result = String::from("You may close this window");
        }
        None => result = String::from("Missing <code> parameter"),
    };

    rocket::Shutdown::notify(shutdown);

    result
}
