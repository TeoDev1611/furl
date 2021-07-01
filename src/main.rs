use clap::{App, Arg};
use ureq::Agent;
use std::time::Duration;

fn main() -> Result<(),ureq::Error>{
    let app = App::new("Furl the Ferris URL Client")
        .version("0.1.0")
        .author("TeoDev1611")
        .about("Furl the Ferris URL Client writed with Rust lang")
        .arg(
            Arg::with_name("url")
                .short("U")
                .long("url")
                .help("Pass the url for the http request")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
      let agent: Agent = ureq::AgentBuilder::new()
          .timeout_read(Duration::from_secs(5))
          .timeout_write(Duration::from_secs(5))
          .build();
    let url = app.value_of("url").unwrap_or("https://httpbin.org/get");
    let req = agent.get(url).call()?.into_string()?;
    println!("{}", req);
    Ok(())
}
