use intaglio::bytes::SymbolTable;

use crate::class;
#[cfg(feature = "core-random")]
use crate::extn::core::random::Random;
use crate::fs;
use crate::interpreter::InterpreterAllocError;
use crate::module;
use crate::sys;

pub mod output;
pub mod parser;
pub mod regexp;

/// Container for interpreter global state.
///
/// A Ruby interpreter requires various pieces of state to execute Ruby code. It
/// needs an object heap, type registry, symbol table, psuedorandom number
/// generator, and more.
///
/// This struct stores all of these components and allows them to be passed
/// around as one bundle. This is useful in FFI contexts because this `State`
/// can be [`Box`]ed and stored in a user data pointer.
#[derive(Default, Debug)]
pub struct State {
    pub parser: Option<parser::State>,
    pub classes: class::Registry,
    pub modules: module::Registry,
    pub vfs: fs::Adapter,
    pub regexp: regexp::State,
    pub symbols: SymbolTable,
    pub output: output::Strategy,
    #[cfg(feature = "core-random")]
    pub prng: Random,
}

impl State {
    /// Create a new `State`.
    ///
    /// The state is comprised of several components:
    ///
    /// - [`Class`] and [`Module`] registries.
    /// - `Regexp` [global state][regexp-state].
    /// - [In-memory virtual filesystem].
    /// - [Ruby parser and file context].
    /// - [Intepreter-level PRNG] (requires activating the `core-random`
    ///   feature).
    /// - [IO capturing] strategy.
    ///
    /// # Errors
    ///
    /// If the `core-random` feature is enabled, this function may return an
    /// error if the interpreter-global psuedorandom number generator fails
    /// to initialize using the platform source of randomness.
    ///
    /// [`Class`]: crate::class_registry::ClassRegistry
    /// [`Module`]: crate::module_registry::ModuleRegistry
    /// [regexp-state]: regexp::State
    /// [In-memory virtual filesystem]: fs
    /// [Ruby parser and file context]: parser::State
    /// [Intepreter-level PRNG]: Random
    /// [IO capturing]: output::Strategy
    pub fn new() -> Result<Self, InterpreterAllocError> {
        Ok(Self {
            parser: None,
            classes: class::Registry::new(),
            modules: module::Registry::new(),
            vfs: fs::Adapter::new(),
            regexp: regexp::State::new(),
            symbols: SymbolTable::new(),
            output: output::Strategy::new(),
            #[cfg(feature = "core-random")]
            prng: Random::new().map_err(|_| InterpreterAllocError::new())?,
        })
    }

    /// Create a new [`parser::State`] from a [`sys::mrb_state`].
    #[doc(hidden)]
    pub(crate) fn try_init_parser(&mut self, mrb: &mut sys::mrb_state) {
        if let Some(parser) = parser::State::new(mrb) {
            if let Some(old_parser) = self.parser.replace(parser) {
                old_parser.close(mrb);
            }
        }
    }
}
