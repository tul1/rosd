use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {

    let rosd_arg = Arg::with_name("ROSLAUNCH_DIR")
        .long("roslaunch-dir")
        .help(
            "Specifies the roslaunch directory to load it/them. Falls back to \
             the ROSLAUNCH_DIR environment variable if unspecified.",
        )
        .env("ROSLAUNCH_DIR")
        .global(true)
        .takes_value(true);

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
        .arg(rosd_arg)
        .subcommand(run_rosd)
        .setting(AppSettings::SubcommandRequiredElseHelp)
}
