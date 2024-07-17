// src/lib.rs
pub mod management_user;
pub mod management_group;
pub mod management_policy;
pub mod management_role;
pub mod management_audit;
pub mod manager_config;
pub mod client;

pub use management_user::UserManager;
pub use management_group::GroupManager;
pub use management_policy::PolicyManager;
pub use management_role::RoleManager;
pub use management_audit::AuditManager;
pub use manager_config::config::Config;
pub use client::MinioClient;
