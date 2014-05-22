pub use self::namever::{
    NameVer
};

pub use self::registry::{
    Registry,
};

pub use self::manifest::{
    Manifest,
    Target,
    TargetKind
};

pub use self::package::{
    Package,
    PackageSet
};

pub use self::source::{
    Source
};

pub use self::summary::{
    Summary
};

pub use self::dependency::Dependency;
pub use self::version_req::VersionReq;

pub mod errors;
pub mod namever;
pub mod source;
pub mod package;
pub mod dependency;
pub mod manifest;
pub mod resolver;
pub mod summary;
mod registry;
mod version_req;