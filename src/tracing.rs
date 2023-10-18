use std::{
    fmt,
    sync::{Arc, Mutex, OnceLock},
};

use matrix_sdk_common::js_tracing::{JsLoggingSubscriber, MakeJsLogWriter};
use tracing::Level;
use tracing_subscriber::{filter::LevelFilter, prelude::*, reload};
use wasm_bindgen::prelude::*;

/// Logger level.
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum LoggerLevel {
    /// `TRACE` level.
    ///
    /// Designate very low priority, often extremely verbose,
    /// information.
    Trace,

    /// `DEBUG` level.
    ///
    /// Designate lower priority information.
    Debug,

    /// `INFO` level.
    ///
    /// Designate useful information.
    Info,

    /// `WARN` level.
    ///
    /// Designate hazardous situations.
    Warn,

    /// `ERROR` level.
    ///
    /// Designate very serious errors.
    Error,
}

/// Internal state.
///
/// This is a singleton: there is at most one instance in the entire process
struct TracingInner {
    /// The log level last set by `min_level`
    level: Level,

    level_filter_reload_handle: reload::Handle<LevelFilter, JsLoggingSubscriber>,
}

/// Type to install and to manipulate the tracing layer.
#[wasm_bindgen]
pub struct Tracing {
    inner: Arc<Mutex<TracingInner>>,
}

impl fmt::Debug for Tracing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tracing").finish_non_exhaustive()
    }
}

#[wasm_bindgen]
impl Tracing {
    /// Check whether the `tracing` feature has been enabled.
    ///
    /// @deprecated: `tracing` is now always enabled.
    #[wasm_bindgen(js_name = "isAvailable")]
    pub fn is_available() -> bool {
        true
    }

    fn install_or_get_inner() -> Arc<Mutex<TracingInner>> {
        static INSTALL: OnceLock<Arc<Mutex<TracingInner>>> = OnceLock::new();

        // if this is the first Tracing to be created, create the TracingInner singleton
        // and stash it in `INSTALL`
        INSTALL
            .get_or_init(|| {
                let format = tracing_subscriber::fmt::format().without_time().pretty();
                let subscriber = tracing_subscriber::fmt()
                    .with_max_level(Level::TRACE)
                    .with_writer(MakeJsLogWriter::new())
                    .with_ansi(false)
                    .event_format(format)
                    .finish();

                let (level_filter, level_filter_reload_handle) =
                    reload::Layer::new(LevelFilter::OFF);
                subscriber.with(level_filter).init();

                Arc::new(Mutex::new(TracingInner {
                    level: Level::ERROR,
                    level_filter_reload_handle,
                }))
            })
            .clone()
    }

    /// Install the tracing layer.
    #[wasm_bindgen(constructor)]
    pub fn new(min_level: LoggerLevel) -> Result<Tracing, JsError> {
        let tracing = Tracing { inner: Tracing::install_or_get_inner() };
        tracing.min_level(min_level)?;

        Ok(tracing)
    }

    /// Re-define the minimum logger level.
    #[wasm_bindgen(setter, js_name = "minLevel")]
    pub fn min_level(&self, min_level: LoggerLevel) -> Result<(), JsError> {
        let mut inner = self.inner.lock()?;
        // we store the level in `inner.level`, so that `turn_on` knows what to restore
        // it to.
        inner.level = min_level.into();
        inner
            .level_filter_reload_handle
            .modify(|filter| *filter = LevelFilter::from_level(inner.level))?;
        Ok(())
    }

    /// Turn the logger on, i.e. it emits logs again if it was turned
    /// off.
    #[wasm_bindgen(js_name = "turnOn")]
    pub fn turn_on(&self) -> Result<(), JsError> {
        let inner = self.inner.lock()?;
        inner
            .level_filter_reload_handle
            .modify(|filter| *filter = LevelFilter::from_level(inner.level))?;
        Ok(())
    }

    /// Turn the logger off, i.e. it no longer emits logs.
    #[wasm_bindgen(js_name = "turnOff")]
    pub fn turn_off(&self) -> Result<(), JsError> {
        let inner = self.inner.lock()?;
        inner.level_filter_reload_handle.modify(|filter| *filter = LevelFilter::OFF)?;
        Ok(())
    }
}

impl From<LoggerLevel> for Level {
    fn from(value: LoggerLevel) -> Self {
        use LoggerLevel::*;

        match value {
            Trace => Self::TRACE,
            Debug => Self::DEBUG,
            Info => Self::INFO,
            Warn => Self::WARN,
            Error => Self::ERROR,
        }
    }
}
