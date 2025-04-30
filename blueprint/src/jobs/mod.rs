mod create_workspace;
mod destroy_workspace;

pub use create_workspace::{CreateWorkspaceParams, ResourceTier, create_workspace};
pub use destroy_workspace::destroy_workspace;

// Re-export job IDs
pub const CREATE_WORKSPACE_JOB_ID: u32 = 0;
pub const DESTROY_WORKSPACE_JOB_ID: u32 = 1;
