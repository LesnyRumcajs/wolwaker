use clap::{App, Arg};
use iron::{prelude::*, status};
use log::{info, trace, warn};
use router::Router;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use wakey::WolPacket;

struct Config {
    mac: String,
    port: u32,
    verbosity: log::Level,
}

fn parse_args() -> Config {
    let matches = App::new("Wolwaker")
        .about("Simple service for sending Wake-on-LAN packets by GET requests")
        .arg(
            Arg::with_name("mac")
                .short("m")
                .value_name("MAC")
                .help("MAC address to send WOL to")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .value_name("PORT")
                .help("Listening port for the service")
                .default_value("3333"),
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Sets the level of log verbosity"),
        )
        .get_matches();

    let mac = matches.value_of("mac").unwrap();
    let port: u32 = matches
        .value_of("port")
        .unwrap()
        .parse()
        .expect("Incorrect port");
    let verbosity = match matches.occurrences_of("verbosity") {
        0 => log::Level::Info,
        1 => log::Level::Debug,
        _ => log::Level::Trace,
    };

    Config {
        mac: String::from(mac),
        port,
        verbosity,
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let config = parse_args();

    simple_logger::init_with_level(config.verbosity).unwrap();
    trace!("MAC: {:?}", config.mac);
    trace!("Listening port: {:?}", config.port);
    trace!("Verbosity: {}", config.verbosity);

    let wol_packet = Arc::new(Mutex::new(WolPacket::from_string(&config.mac, ':')));
    let packet = wol_packet.clone();
    let mut router = Router::new();

    router.get(
        "/wake",
        move |request: &mut Request| get_wake(request, &packet),
        "wake",
    );

    info!("Serving on http://localhost:{}...", config.port);
    Iron::new(router).http(SocketAddr::from(([0, 0, 0, 0], config.port as u16)))?;

    Ok(())
}

fn get_wake(
    _request: &mut Request,
    wol_packet: &std::sync::Arc<std::sync::Mutex<wakey::WolPacket>>,
) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(
        "text/html; charset=utf-8"
            .parse::<iron::mime::Mime>()
            .unwrap(),
    );

    match wol_packet.lock().unwrap().send_magic() {
        Ok(_) => {
            info!("Magic packet sent!");
            response.set_mut(
                r#"
                <title>wolwaker</title>
                <body>Everything went alright!</body>
                "#,
            );
        }
        Err(_) => {
            response.set_mut(
                r#"
                <title>wolwaker</title>
                <body>Failed miserably. :(</body>
                "#,
            );
            warn!("Unable to send the magic packet!")
        }
    };

    Ok(response)
}
