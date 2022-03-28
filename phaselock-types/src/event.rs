//! Events that a `PhaseLock` instance can emit

use std::sync::Arc;

use crate::{data::Stage, error::PhaseLockError};

/// A status event emitted by a `PhaseLock` instance
///
/// This includes some metadata, such as the stage and view number that the event was generated in,
/// as well as an inner [`EventType`] describing the event proper.
#[derive(Clone, Debug)]
pub struct Event<B: Send + Sync, S: Send + Sync> {
    /// The view number that this event originates from
    pub view_number: u64,
    /// The stage that this event originates from
    pub stage: Stage,
    /// The underlying event
    pub event: EventType<B, S>,
}

/// The type and contents of a status event emitted by a `PhaseLock` instance
///
/// This enum does not include metadata shared among all variants, such as the stage and view
/// number, and is thus always returned wrapped in an [`Event`].
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum EventType<B: Send + Sync, S: Send + Sync> {
    /// A view encountered an error and was interrupted
    Error {
        /// The underlying error
        error: Arc<PhaseLockError>,
    },
    /// A new block was proposed
    Propose {
        /// The block that was proposed
        block: Arc<B>,
    },
    /// A new decision event was issued
    Decide {
        /// The list of blocks that were committed by this decision
        ///
        /// This list is sorted in reverse view number order, with the newest (highest view number)
        /// block first in the list.
        ///
        /// This list may be incomplete if the node is currently performing catchup.
        block: Arc<Vec<B>>,
        /// The list of states that were committed by this decision
        ///
        /// This list is sorted in reverse view number order, with the newest (highest view number)
        /// state first in the list.
        ///
        /// This list may be incomplete if the node is currently performing catchup.
        state: Arc<Vec<S>>,
    },
    /// A new view was started by this node
    NewView {
        /// The view being started
        view_number: u64,
    },
    /// A view was canceled by a timeout interrupt
    ViewTimeout {
        /// The view that timed out
        view_number: u64,
    },
    /// This node is the leader for this view
    Leader {
        /// The current view number
        view_number: u64,
    },
    /// This node is a follower for this view
    Follower {
        /// The current view number
        view_number: u64,
    },

    /// The node has been synced with the network
    Synced {
        /// The current view number
        view_number: u64,
    },
}
