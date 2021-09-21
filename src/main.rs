extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("organizer")
        .version("1.0")
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("mount_point")
                .short("m")
                .long("mount-point")
                .takes_value(true)
                .value_name("MOUNT_POINT")
                .required(true)
                .help("Mount point path."),
        )
        .get_matches();

    println!(
        "Using mount point: {}",
        matches.value_of("mount_point").unwrap()
    );
}
