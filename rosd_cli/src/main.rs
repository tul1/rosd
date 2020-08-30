mod cli;


use clap::{ArgMatches};
use rosd_lib::rosd_run;


fn main() {

    let matches = cli::build_cli().get_matches();
    match matches.subcommand() {
        ("run", Some(matches)) => run_rosd(matches),
        _ => unreachable!("The cli parser should prevent reaching here"),
    }

}


fn run_rosd(matches: &ArgMatches) {
    println!("{:?}", matches);
    rosd_run();
}
