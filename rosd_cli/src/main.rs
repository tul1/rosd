mod cli;


use clap::{ArgMatches};
use rosd::rosd_run;


fn main() {

    let matches = cli::build_cli().get_matches();
    match matches.subcommand() {
        ("logs", Some(matches)) => logs_rosd(matches).unwrap_or_else(handle_error),
        ("services", Some(matches)) => services_rosd(matches).unwrap_or_else(handle_error),
        ("run", Some(matches)) => run_rosd(matches).unwrap_or_else(handle_error),
        _ => unreachable!("The cli parser should prevent reaching here"),
    }

}


fn run_rosd(matches: &ArgMatches) {
    println!("{:?}", matches);
    rosd_run();
    Ok(())
}


fn logs_rosd(matches: &ArgMatches) {
    println!("{:?}", matches);
    Ok(())
}

fn services_rosd(matches: &ArgMatches) {
    println!("{:?}", matches);
    Ok(())
}
