// TODO: Imports here instad of the root module?

pub use crate::protocol::{EntryId, Path};
pub use crate::providers::LogProvider;
pub use once_cell::sync::Lazy;

pub fn split_module_path(module_path: &'static str) -> Path {
    let path: Vec<_> = module_path.split("::").map(EntryId::from).collect();
    Path::from(path)
}

#[macro_export]
macro_rules! provider {
    () => {
        $crate::provider!(@public false);
    };
    (public) => {
        $crate::provider!(@public true);
    };
    (@public $public:expr) => {
        pub static RILL: $crate::macros::Lazy<$crate::macros::LogProvider> =
            $crate::macros::Lazy::new(|| {
                let name = std::module_path!();
                let path = $crate::macros::split_module_path(name);
                let provider = $crate::macros::LogProvider::new(path);
                if $public {
                    provider.export();
                }
                provider
            });
    };
}

#[macro_export]
macro_rules! log {
    ($msg:expr) => {{
        {
            let rill = $crate::macros::Lazy::force(&RILL);
            if rill.is_active() {
                rill.log($msg, None);
            }
        }
    }};
}
