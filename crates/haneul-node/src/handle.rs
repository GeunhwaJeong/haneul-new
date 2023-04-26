// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! HaneulNodeHandle wraps HaneulNode in a way suitable for access by test code.
//!
//! When starting a HaneulNode directly, in a test (as opposed to using Swarm), the node may be
//! running inside of a simulator node. It is therefore a mistake to do something like:
//!
//! ```ignore
//!     use test_utils::authority::{start_node, spawn_checkpoint_processes};
//!
//!     let node = start_node(config, registry).await;
//!     spawn_checkpoint_processes(config, &[node]).await;
//! ```
//!
//! Because this would cause the checkpointing processes to be running inside the current
//! simulator node rather than the node in which the HaneulNode is running.
//!
//! HaneulNodeHandle provides an easy way to do the right thing here:
//!
//! ```ignore
//!     let node_handle = start_node(config, registry).await;
//!     node_handle.with_async(|haneul_node| async move {
//!         spawn_checkpoint_processes(config, &[haneul_node]).await;
//!     });
//! ```
//!
//! Code executed inside of with or with_async will run in the context of the simulator node.
//! This allows tests to break the simulator abstraction and magically mutate or inspect state that
//! is conceptually running on a different "machine", but without producing extremely confusing
//! behavior that might result otherwise. (For instance, any network connection that is initiated
//! from a task spawned from within a with or with_async will appear to originate from the correct
//! simulator node.
//!
//! It is possible to exfiltrate state:
//!
//! ```ignore
//!    let state = node_handle.with(|haneul_node| haneul_node.state);
//!    // DO NOT DO THIS!
//!    do_stuff_with_state(state)
//! ```
//!
//! We can't prevent this completely, but we can at least make the right way the easy way.

use super::HaneulNode;
use std::future::Future;
use std::sync::Arc;

/// Wrap HaneulNode to allow correct access to HaneulNode in simulator tests.
pub struct HaneulNodeHandle(Option<Arc<HaneulNode>>);

impl HaneulNodeHandle {
    pub fn new(node: Arc<HaneulNode>) -> Self {
        Self(Some(node))
    }

    fn inner(&self) -> &Arc<HaneulNode> {
        self.0.as_ref().unwrap()
    }

    pub fn with<T>(&self, cb: impl FnOnce(&HaneulNode) -> T) -> T {
        let _guard = self.guard();
        cb(self.inner())
    }
}

#[cfg(not(msim))]
impl HaneulNodeHandle {
    // Must return something to silence lints above at `let _guard = ...`
    fn guard(&self) -> u32 {
        0
    }

    pub async fn with_async<'a, F, R, T>(&'a self, cb: F) -> T
    where
        F: FnOnce(&'a HaneulNode) -> R,
        R: Future<Output = T>,
    {
        cb(self.inner()).await
    }
}

#[cfg(msim)]
impl HaneulNodeHandle {
    fn guard(&self) -> haneul_simulator::runtime::NodeEnterGuard {
        self.inner().sim_node.enter_node()
    }

    pub async fn with_async<'a, F, R, T>(&'a self, cb: F) -> T
    where
        F: FnOnce(&'a HaneulNode) -> R,
        R: Future<Output = T>,
    {
        let fut = cb(self.0.as_ref().unwrap());
        self.inner().sim_node.await_future_in_node(fut).await
    }
}

#[cfg(msim)]
impl Drop for HaneulNodeHandle {
    fn drop(&mut self) {
        let node_id = self.inner().sim_node.id();
        // Shut down the sim node, but only if we were the last holder of a reference to the haneul
        // node.
        let haneul_node_arc = self.0.take().unwrap();
        let haneul_node = Arc::downgrade(&haneul_node_arc);
        drop(haneul_node_arc);
        if haneul_node.upgrade().is_none() {
            haneul_simulator::runtime::Handle::try_current().map(|h| h.delete_node(node_id));
        }
    }
}

impl From<Arc<HaneulNode>> for HaneulNodeHandle {
    fn from(node: Arc<HaneulNode>) -> Self {
        HaneulNodeHandle::new(node)
    }
}
