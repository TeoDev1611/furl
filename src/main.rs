use clap::{App, Arg};
use colored::*;
use colored_json::ToColoredJson;
use std::time::Duration;
use ureq::Agent;

fn main() -> Result<(), ureq::Error> {
    let app = App::new("Furl the Ferris URL Client")
        .version("0.1.0")
        .author("TeoDev1611")
        .about("Furl the Ferris URL Client writed with Rust lang")
        .arg(
            Arg::with_name("url")
                .short("U")
                .long("url")
                .help("Pass the url for the http request")
                .takes_value(true), //.required(true),
        )
        .arg(
            Arg::with_name("header")
                .short("H")
                .long("Header")
                .help("Pass the header for the http request")
                .takes_value(true),
        )
        .get_matches();
    let agent: Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();
    let url = app.value_of("url").unwrap_or("https://httpbin.org/get");
    let headers = app
        .value_of("header")
        .unwrap_or("Application-json")
        .split("-");

    let vec = headers.collect::<Vec<&str>>();
    let req = agent.get(url).set(vec[0], vec[1]).call()?.into_string()?;
    println!("{}", "RESPONSE DATA: \n".green());
    color_json(req);
    Ok(())
}

fn color_json(text: String) -> ::std::result::Result<(), Box<::std::error::Error>> {
    println!("{}", text.to_colored_json_auto()?);
    Ok(())
}
