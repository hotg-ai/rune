//! This module contains the implementation of the MovingState component.

/// The struct's aim is to cut the proccess of moving into
/// small pieces with states.
#[derive(Debug)]
pub enum MovingState {
    /// This is the default state, when no moving is happening.
    /// When should move the next state is `SetMoving` with
    /// `starting` set to `true`.
    Idle,
    /// At this state, checking which tile should move.
    /// When done checking, if some tiles should move,
    /// the next state is `Animating`
    /// otherwise, the next state is `Finishing` with
    /// `moved` set to `!starting`.
    SetMoving {
        /// Tells if this is the first time checking for moving tiles.
        starting: bool,
    },
    /// While at this state, all the tiles that should move are
    /// sliding in the moving direction.
    /// When done animating, the next state is `Merging`.
    Animating,
    /// At this state, checking each tiles that are at the same position,
    /// Are being merged.
    /// Then setting the next state to `SetMoving` with `starting` set to `false`.
    Merging,
    /// At this state, all the tiles are at their final position.
    /// Removing the merged compoent from the tiles and spawning a new
    /// tile if `moved` is `true`.
    /// When done, the next state is `CheckingMoveable`.
    Finishing {
        /// Tells if any tile have been moved.
        moved: bool,
    },
    /// At this state, the movement is already done.
    /// Checking if there are no moves and the game is over.
    /// When done the next state is `Idle`.
    CheckingMoveable,
}

impl Default for MovingState {
    /// Creates an Idle moving state.
    fn default() -> Self {
        Self::Idle
    }
}
