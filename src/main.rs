//! `mrsa` provides Arch Linux build and package automation
//!
//! ## About
//!
//! `mrsa` provide Arch Linux build and package automation
use clap::{App, AppSettings, Arg, SubCommand};
use fungus::prelude::*;
use mrsa::*;
use std::ffi::OsString;

/// CLI provides an abstraction layer for testing the cli
#[derive(Debug, PartialEq)]
struct CLI {}
impl CLI {
    /// Create a new instance of the cli with the command line arguments
    pub fn new() -> Result<Self> {
        Self::new_from(std::env::args_os().into_iter())
    }

    /// Create a new instance of the cli with the given arguments
    fn new_from<T, U>(args: T) -> Result<Self>
    where
        T: Iterator<Item = U>,
        U: Into<OsString> + Clone,
    {
        Logger::init()?;
        Logger::enable_buffer();

        // About strings
        let use_about = r"Persist configuration across runs

Examples:

  # Use and persist the 'base' profile
  mrsa use profile base

  # Use and persist a custom profile '~/foo.yaml'
  mrsa use profile ~/foo.yaml
";

        // Parse cli args
        let matches = App::new(APP_NAME)
            .version(&format!("v{}", APP_VERSION)[..])
            .about(APP_DESCRIPTION)
            .setting(AppSettings::SubcommandRequiredElseHelp)

            // Global arguments
            // -----------------------------------------------------------------------------------------
            .arg(Arg::with_name("test").short("t").long("test").help("Enable test mode"))
            .arg(Arg::with_name("debug").short("d").long("debug").help("Enable debug logging"))
            .arg(Arg::with_name("quiet").short("q").long("quiet").help("Disable all logging"))

            // log-level - configures the log level to use
            .arg(Arg::with_name("loglevel").long("log-level").value_name("NAME").takes_value(true)
                .help("Sets the log level [error|warn|info|debug|trace] [default: info]"))

            // config-dir - is where mrsa persists its configuration
            .arg(Arg::with_name("config_dir").long("config-dir").value_name("PATH").takes_value(true)
                .help("Sets the config directory [default: $XDG_CONFIG_HOME/mrsa]"))

            // data-dir - is where all repos are downloaded and all work is done
            .arg(Arg::with_name("data_dir").long("data-dir").value_name("PATH").takes_value(true)
                .help("Sets the data directory [default: $XDG_DATA_HOME/mrsa]"))

            // Version command
            // -----------------------------------------------------------------------------------------
            .subcommand(SubCommand::with_name("version").alias("v").alias("ver").about("Print version information"))

            // Use command
            // -----------------------------------------------------------------------------------------
            .subcommand(SubCommand::with_name("use").about("Persist configuration across runs").long_about(use_about)
                .subcommand(SubCommand::with_name("profile").about("Use the given profile and persist the change")
                    .arg(Arg::with_name("profile_arg").index(1).required(true).value_names(&["NAME/PATH"])
                        .help("Profile name or path to use and persist"))),
            )

            // Remove command
            // -----------------------------------------------------------------------------------------
            .subcommand(SubCommand::with_name("remove").alias("rm").about("Remove various mrsa components")
                .subcommand(SubCommand::with_name("config").about("Remove the persisted configuration"))
                    .subcommand(SubCommand::with_name("repos").alias("repo").about("Remove indicated locally cloned repos")
                        .arg(Arg::with_name("repos_arg").index(1).required(true).value_names(&["all, aur, boot, config, profiles"])
                            .multiple(true).help("Repo name/s to remove"))),
            )

            .get_matches_from_safe(args)?;

        // Set incoming arguments
        // ---------------------------------------------------------------------------------------------
        let mut mrsa = Mrsa::new();
        if matches.is_present("loglevel") {
            mrsa.loglevel_str(matches.value_of("loglevel").unwrap()); // call loglevel first to let debug override
        }
        if matches.is_present("config_dir") {
            mrsa.config_dir(matches.value_of("config_dir").unwrap())?;
        }
        if matches.is_present("data_dir") {
            mrsa.data_dir(matches.value_of("data_dir").unwrap())?;
        }
        if matches.is_present("debug") {
            mrsa.debug(matches.value_of("debug").unwrap().to_lowercase().parse()?);
        }
        if matches.is_present("quiet") {
            mrsa.quiet(matches.value_of("quiet").unwrap().to_lowercase().parse()?);
        }
        if matches.is_present("test") {
            mrsa.test(matches.value_of("test").unwrap().to_lowercase().parse()?);
        }

        // Execute version
        // ---------------------------------------------------------------------------------------------
        if let Some(ref _matches) = matches.subcommand_matches("version") {
            println!("MRSA CLI - {}", APP_DESCRIPTION);
            println!("{:->w$}", "-", w = 60);
            println!("{:<w$} {}", "Version:", APP_VERSION, w = 18);
            println!("{:<w$} {}", "Build Date:", APP_BUILD_DATE, w = 18);
            println!("{:<w$} {}", "Git Commit:", APP_GIT_COMMIT, w = 18);
        }

        // Execute use command before initializing mrsa to to update config first
        // ---------------------------------------------------------------------------------------------
        if let Some(ref _matches) = matches.subcommand_matches("use") {
            // Simply print out current persisted configuration
            return Err(PathError::parent_not_found("blah").into());
        }

        // Execute remove
        // ---------------------------------------------------------------------------------------------
        if let Some(ref matches) = matches.subcommand_matches("remove") {
            mrsa.init()?;
            let mut components = Vec::new();
            match matches.subcommand() {
                ("config", Some(_)) => {
                    components.push(Component::Config);
                }
                _ => unreachable!(),
            }

            mrsa.remove(components)?;
        }

        Ok(Self {})
    }
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let cli = CLI::new();
    if cli.is_err() {
        let err = cli.unwrap_err();
        if let Some(err) = err.downcast_ref::<clap::Error>() {
            err.exit();
        } else {
            let fail = err.as_fail();
            panic!("{:?}", fail);
            // for cause in fail.iter_causes() {
            //     println!("\nInfo: caused by {}", cause);
            // }
            // println!("{:?}", fail.backtrace());
        }
    }
}

#[cfg(test)]
mod tests {
    // use fungus::prelude::*;
    // use mrsa::*;

    #[test]
    fn test_main() {
        //let mrsa= Mrsa::new().unwrap().loglevel(log::Level::Trace).init().unwrap();
    }
}
