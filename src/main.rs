#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;

extern crate clap;
use clap::Parser;

const AFTER_HELP: &str = "\
Port/Address/etc can be configured with Rocket's native configuration system
https://rocket.rs/v0.5-rc/guide/configuration/

ROCKET_PORT=80 frisbee";

#[derive(Parser, Debug)]
#[clap(author, version, after_help = AFTER_HELP, long_about = None)]
struct Args {
    /// Directory to serve from disk
    #[clap(short, long, value_parser,
            value_name = "root",
            value_hint = clap::ValueHint::DirPath,
            default_value = ".")]
    root: std::path::PathBuf,

    /// Path prefix for HTTP requests
    #[clap(short, long, value_name = "public", default_value = "/")]
    public: String,
}

#[launch]
fn rocket() -> _ {
    let args = Args::parse();

    rocket::build().mount(args.public, FileServer::from(args.root.as_path()))
}
