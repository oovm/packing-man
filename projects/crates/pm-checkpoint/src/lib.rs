//! 求解断点：保存布局与迭代计数，支持续跑。

mod checkpoint;
mod resume;

pub use checkpoint::{Checkpoint, CHECKPOINT_VERSION};
pub use resume::{
    build_checkpoint_after_run, default_iter_budget, is_resumable, ResumeSession, SolveResume,
};
