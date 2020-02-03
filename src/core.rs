use fungus::prelude::*;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::model::*;

pub const APP_NAME: &'static str = "mrsa";
pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

// Mrsa implementation
// -------------------------------------------------------------------------------------------------
pub struct Mrsa {
    pub(crate) init: bool,
    pub(crate) test: bool,
    pub(crate) test_set: bool,
    pub(crate) debug: bool,
    pub(crate) debug_set: bool,
    pub(crate) quiet: bool,
    pub(crate) quiet_set: bool,
    pub(crate) config_path: PathBuf,
    pub(crate) config_dir: PathBuf,
    pub(crate) config_dir_set: bool,
    pub(crate) data_dir: PathBuf,
    pub(crate) data_dir_set: bool,
    pub(crate) loglevel: log::Level,
    pub(crate) loglevel_set: bool,
    pub(crate) out: Rc<RefCell<dyn io::Write>>,
    config_w: usize, // configuration width to use for output
}
impl Default for Mrsa {
    fn default() -> Self {
        Self {
            init: Default::default(),
            test: Default::default(),
            test_set: Default::default(),
            debug: Default::default(),
            debug_set: Default::default(),
            quiet: Default::default(),
            quiet_set: Default::default(),
            config_path: Default::default(),
            config_dir: Default::default(),
            config_dir_set: Default::default(),
            data_dir: Default::default(),
            data_dir_set: Default::default(),
            loglevel: log::Level::Info,
            loglevel_set: Default::default(),
            out: Rc::new(RefCell::new(io::stdout())),
            config_w: 22,
        }
    }
}
impl Mrsa {
    /// Create a new mrsa instance with defaults.
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    // Property Setters
    // ---------------------------------------------------------------------------------------------

    /// Set test mode with the given `yes` value.
    pub fn test(&mut self, yes: bool) -> &mut Self {
        self.test_set = true;
        self.test = yes;
        self
    }

    /// Set debug mode with the given `yes` value.
    pub fn debug(&mut self, yes: bool) -> &mut Self {
        self.debug_set = true;
        self.debug = yes;
        if yes {
            self.loglevel = log::Level::Debug;
        }
        self
    }

    /// Set quiet mode with the given `yes` value.
    pub fn quiet(&mut self, yes: bool) -> &mut Self {
        self.quiet_set = true;
        self.quiet = yes;
        match yes {
            true => Logger::enable_silence(),
            false => Logger::disable_silence(),
        }
        self
    }

    /// Set the config_dir to use `[default: ~/.config/mrsa]`
    pub fn config_dir<T: AsRef<Path>>(&mut self, path: T) -> Result<&mut Self> {
        self.config_dir_set = true;
        self.config_dir = path.as_ref().abs()?;
        info!("{:>w$} {}", "setting config_dir:", self.config_dir.cyan()?, w = self.config_w);
        self.config_path = self.config_dir.mash(format!("{}.yaml", APP_NAME));
        Ok(self)
    }

    /// Set the data_dir to use `[default: ~/.local/share/mrsa]`
    pub fn data_dir<T: AsRef<Path>>(&mut self, path: T) -> Result<&mut Self> {
        self.data_dir_set = true;
        self.data_dir = path.as_ref().abs()?;
        info!("{:>w$} {}", "setting data_dir:", self.data_dir.cyan()?, w = self.config_w);
        Ok(self)
    }

    /// Set the log level to use from string `level`
    pub fn loglevel_str<T: AsRef<str>>(&mut self, level: T) -> &mut Self {
        self.loglevel_set = true;
        self.loglevel = Logger::level_from_str(level);
        self
    }

    /// Set the log level to use to `level`
    pub fn loglevel(&mut self, level: log::Level) -> &mut Self {
        self.loglevel_set = true;
        self.loglevel = level;
        self
    }

    // Core functions
    // ---------------------------------------------------------------------------------------------

    /// Initialize mrsa with the configured options
    pub fn init(&mut self) -> Result<()> {
        if self.init {
            return Ok(());
        }

        // Configure logging and print out startup
        Logger::init()?;
        Logger::set_level(self.loglevel);
        match self.test {
            true => Logger::enable_buffer(),
            false => Logger::disable_buffer(),
        }
        info!("{}", format!("<<{{ {} v{} }}>>", APP_NAME, APP_VERSION).green().bold());
        Logger::flush_buffer();

        // Configure mrsa
        self.load_config()?;

        self.init = true;
        Ok(())
    }

    /// Save the current configuration. By default this will be at
    /// $XDG_CONFIG_HOME/mrsa/mrsa.yaml unless overridden.
    pub fn save_config(&self) -> Result<()> {
        info!("{}{}", "Persisting configuration: ".yellow().bold(), self.config_path.cyan()?);
        let config = Config::new();
        config.save(&self.config_path)?;
        Ok(())
    }

    /// Remove the given `components`
    pub fn remove<T: AsRef<[Component]>>(&mut self, components: T) -> Result<()> {
        if components.as_ref().iter().any(|x| x == &Component::None) {
            return Err(ComponentError::InvalidName.into());
        }
        let mut component_vec = components.as_ref().iter().map(|x| x.to_owned()).collect();

        // Remove all components
        // -----------------------------------------------------------------------------------------
        if components.as_ref().iter().any(|x| x == &Component::All) {
            component_vec = Component::all();
        }

        info!("{}{}", "Removing components: ".yellow().bold(), Component::join(&component_vec).cyan());
        for component in component_vec {
            let path = match component {
                Component::Config => &self.config_dir,
                _ => continue,
            };
            if sys::exists(path) {
                info!("Removing: {}", path.to_string()?.cyan());
                sys::remove_all(&path)?;
            } else {
                info!("Doesn't exist: {}", path.to_string()?.cyan());
            }
        }
        Ok(())
    }

    // Private functions
    // ---------------------------------------------------------------------------------------------

    // Load the current configuration
    pub(crate) fn load_config(&mut self) -> Result<()> {
        info!("{}", "Loading configuration".yellow().bold());

        // Config dir by default is $XDG_CONFIG_HOME/mrsa/mrsa.yaml
        if !self.config_dir_set {
            self.config_dir = user::config_dir()?.mash("mrsa");
            self.config_path = self.config_dir.mash(format!("{}.yaml", APP_NAME));
            info!("{:>w$} {}", "defaulting config_dir:", self.config_dir.cyan()?, w = self.config_w);
        }

        // Data dir by default is $XDG_DATA_HOME/mrsa
        if !self.data_dir_set {
            self.data_dir = user::data_dir()?.mash("mrsa");
            info!("{:>w$} {}", "defaulting data_dir:", self.data_dir.cyan()?, w = self.config_w);
        }
        Ok(())
    }

    // Implement support for write*! macro varients to use Mrsa as a Writer.
    // We actually don't need to implement the entire fmt::Write trait only this func
    // as macros don't seem to honor the full trait contractd only existance of the func.
    pub fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) {
        if !self.quiet {
            self.out.borrow_mut().write_fmt(fmt).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test setup
    fn setup<T: AsRef<Path>>(path: T) -> PathBuf {
        let temp = PathBuf::from("tests/temp").abs().unwrap();
        sys::mkdir(&temp).unwrap();
        temp.mash(path.as_ref())
    }
}
