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
        .subcommand(
            clap::Command::new("cloud")
                .about("Subcommands related to cloud storage")
                .subcommand(clap::Command::new("auth")),
        )
        .subcommand(clap::Command::new("local").about("about").alias("l"))
        .get_matches();

    if let Some(_sub_m) = matches.subcommand_matches("cloud") {
        if let Some(_sub_m) = _sub_m.subcommand_matches("auth") {
            handle_cloud_auth().await?;
        }
    }

    if let Some(_sub_m) = matches.subcommand_matches("local") {
        print!("local")
    }

    Ok(())
}

// fn is_authenticated() -> bool {
//     let code = std::fs::read_to_string(config::AUTH_CODE_PATH).unwrap();
// }

async fn handle_cloud_auth() -> Result<(), rocket::Error> {
    println!("Taking you to the Google Auth page...");

    let url = format!(
        "{}?client_id={}&redirect_uri={}&scope={}&response_type=code&access_type=offline",
        config::GOOGLE_AUTH_ENDPOINT,
        config::GOOGLE_OAUTH_CLIENT_ID,
        config::GOOGLE_REDIRECT_URI,
        config::GOOGLE_SCOPE,
    );

    open::that(url).unwrap();

    rocket::build()
        .mount("/", routes![google_oath2_callback])
        .launch()
        .await?;

    // let code = std::fs::read_to_string(config::AUTHCODE_FILE_PATH).unwrap();
    // println!("Code: {}", code);

    Ok(())
}

#[get("/oauth2/google?<code>")]
async fn google_oath2_callback(shutdown: rocket::Shutdown, code: Option<String>) -> &'static str {
    match code {
        Some(val) => {
            let _token = get_access_token(val).await.unwrap();
        }
        None => panic!("Parameter <code> is missing"),
    };

    // std::fs::write(config::AUTH_CODE_PATH, val).unwrap();

    rocket::Shutdown::notify(shutdown);
    "You may close the window"
}

async fn get_access_token(code: String) -> Result<&'static str, reqwest::Error> {
    let url = format!(
        "{}?code={}&client_id={}&client_secret={}&redirect_uri={}&grant_type=authorization_code",
        config::GOOGLE_TOKEN_ENDPOINT,
        code,
        config::GOOGLE_OAUTH_CLIENT_ID,
        config::GOOGLE_OAUTH_CLIENT_SECRET,
        config::GOOGLE_REDIRECT_URI
    );

    let client = reqwest::Client::new();
    let res = client.post(url).header("Content-Length", 0).send().await?;
    println!("{:#?}", res);
    // let data = res
    //     .json::<std::collections::HashMap<String, String>>()
    //     .await?;
    let data = res.text().await?;

    println!("{:#?}", data);

    Ok("test")

    // match res {
    //     Ok(val) => println!("{:?}", val),
    //     Err(e) => println!("Error retrieving an access token: {:?}", e),
    // }
}
