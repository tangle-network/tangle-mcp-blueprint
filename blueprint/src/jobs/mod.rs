mod create_project;
mod destroy_project;

pub use create_project::{CreateProjectParams, ResourceTier, create_project};
pub use destroy_project::{DestroyProjectParams, destroy_project};

// Re-export job IDs
pub const CREATE_PROJECT_JOB_ID: u32 = 1;
pub const DESTROY_PROJECT_JOB_ID: u32 = 2;
