use crate::util::Id;

/// Violation of [`ArgMatches`][crate::ArgMatches] assumptions
#[derive(Clone, Debug)]
#[allow(missing_copy_implementations)] // We might add non-Copy types in the future
#[non_exhaustive]
pub enum MatchesError {
    /// Failed to downcast `AnyValue` to the specified type
    #[non_exhaustive]
    Downcast {
        /// Type for value stored in [`ArgMatches`][crate::ArgMatches]
        actual: super::AnyValueId,
        /// The target type to downcast to
        expected: super::AnyValueId,
    },
    /// Argument not defined in [`Command`][crate::Command]
    #[non_exhaustive]
    UnknownArgument {
        // Missing `id` but blocked on a public id type which will hopefully come with `unstable-v4`
    },
}

impl MatchesError {
    #[track_caller]
    pub(crate) fn unwrap<T>(id: &Id, r: Result<T, MatchesError>) -> T {
        let err = match r {
            Ok(t) => {
                return t;
            }
            Err(err) => err,
        };
        panic!(
            "Mismatch between definition and access of `{:?}`. {}",
            id, err
        )
    }
}

impl std::error::Error for MatchesError {}

impl std::fmt::Display for MatchesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Downcast { actual, expected } => {
                writeln!(
                    f,
                    "Could not downcast to {:?}, need to downcast to {:?}",
                    expected, actual
                )
            }
            Self::UnknownArgument {} => {
                writeln!(f, "Unknown argument or group id.  Make sure you are using the argument id and not the short or long flags")
            }
        }
    }
}

#[test]
fn check_auto_traits() {
    static_assertions::assert_impl_all!(
        MatchesError: Send,
        Sync,
        std::panic::RefUnwindSafe,
        std::panic::UnwindSafe,
        Unpin
    );
}
