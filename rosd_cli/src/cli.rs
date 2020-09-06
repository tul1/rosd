use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {

    let roslaunch_dir_arg = Arg::with_name("ROSLAUNCH_DIR")
        .long("roslaunch-dir")
        .help(
            "Specifies the roslaunch directory to load it/them. Falls back to \
             the ROSLAUNCH_DIR environment variable if unspecified.",
        )
        .env("ROSLAUNCH_DIR")
        .global(true)
        .takes_value(true)
        .required(true);

    let logs_rosd = SubCommand::with_name("logs")
        .about(
            "Show ros node logs",
        )
        .arg(
            Arg::with_name("tail")
                .long("tail")
                .takes_value(true)
                .number_of_values(1)
                .help("Number of lines to show from the end of the logs."),
        )
        .arg(
            Arg::with_name("ros_node")
                .long("ros node")
                .takes_value(true)
                .number_of_values(1)
                .help("Ros node")
                .required(true),
        );
 
    let services_rosd = SubCommand::with_name("services")
        .about(
            "rosd systemd services",
        )
        .arg(
            Arg::with_name("list")
                .help("List all rosd services."),
        )
        .arg(
            Arg::with_name("show")
                .long("rosd service name")
                .takes_value(true)
                .number_of_values(1)
                .help("rosd service name")
        )
        .arg(
            Arg::with_name("remove")
                .long("service name")
                .takes_value(true)
                .number_of_values(1)
                .help("Service name")
        );

        
    let run_rosd = SubCommand::with_name("run")
        .about(
            "Creates the systemd tree directory, services and launch \
             every ros nodes",
        );

    App::new("rosd")
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::VersionlessSubcommands)
        .after_help(
            "You can also run `rosd SUBCOMMAND -h` to get more information about that subcommand.",
        )
        .arg(roslaunch_dir_arg)
        .subcommand(logs_rosd)
        .subcommand(services_rosd)
        .subcommand(run_rosd)
        .setting(AppSettings::SubcommandRequiredElseHelp)
}
