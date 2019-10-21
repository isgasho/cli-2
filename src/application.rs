//! OpenLibra Abscissa Application

use crate::{commands::OpenLibraCmd, config::AppConfig};
use abscissa_core::{
    application, config, logging, Application, EntryPoint, FrameworkError, StandardPaths,
};
use lazy_static::lazy_static;

lazy_static! {
    /// Application state
    pub static ref APPLICATION: application::Lock<OpenLibraApp> = application::Lock::default();
}

/// Obtain a read-only (multi-reader) lock on the application state.
///
/// Panics if the application state has not been initialized.
pub fn app_reader() -> application::lock::Reader<OpenLibraApp> {
    APPLICATION.read()
}

/// Obtain an exclusive mutable lock on the application state.
pub fn app_writer() -> application::lock::Writer<OpenLibraApp> {
    APPLICATION.write()
}

/// Obtain a read-only (multi-reader) lock on the application configuration.
///
/// Panics if the application configuration has not been loaded.
pub fn app_config() -> config::Reader<OpenLibraApp> {
    config::Reader::new(&APPLICATION)
}

/// OpenLibra Application
#[derive(Debug)]
pub struct OpenLibraApp {
    /// Application configuration.
    config: Option<AppConfig>,

    /// Application state.
    state: application::State<Self>,
}

/// Initialize a new application instance.
///
/// By default no configuration is loaded, and the framework state is
/// initialized to a default, empty state (no components, threads, etc).
impl Default for OpenLibraApp {
    fn default() -> Self {
        Self {
            config: None,
            state: application::State::default(),
        }
    }
}

impl Application for OpenLibraApp {
    /// Entrypoint command for this application.
    type Cmd = EntryPoint<OpenLibraCmd>;

    /// Application configuration.
    type Cfg = AppConfig;

    /// Paths to resources within the application.
    type Paths = StandardPaths;

    /// Accessor for application configuration.
    fn config(&self) -> &AppConfig {
        self.config.as_ref().expect("config not loaded")
    }

    /// Borrow the application state immutably.
    fn state(&self) -> &application::State<Self> {
        &self.state
    }

    /// Borrow the application state mutably.
    fn state_mut(&mut self) -> &mut application::State<Self> {
        &mut self.state
    }

    /// Register all components used by this application.
    fn register_components(&mut self, command: &Self::Cmd) -> Result<(), FrameworkError> {
        let components = self.framework_components(command)?;
        self.state.components.register(components)
    }

    /// Post-configuration lifecycle callback.
    fn after_config(&mut self, config: Self::Cfg) -> Result<(), FrameworkError> {
        // Configure components
        self.state.components.after_config(&config)?;
        self.config = Some(config);
        Ok(())
    }

    /// Get logging configuration from command-line options
    fn logging_config(&self, command: &EntryPoint<OpenLibraCmd>) -> logging::Config {
        if command.verbose {
            logging::Config::verbose()
        } else {
            logging::Config::default()
        }
    }
}
