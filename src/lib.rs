// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Diagnostics for an Oxide sled that exposes common support commands.

use futures::{stream::FuturesUnordered, StreamExt};

mod queries;
pub use queries::SupportBundleCommandHttpOutput;
use queries::*;

#[cfg(target_os = "illumos")]
mod contract;

#[cfg(not(target_os = "illumos"))]
use std::process::Command;

/// List all zones on a sled.
pub async fn zoneadm_info(
) -> Result<SupportBundleCmdOutput, SupportBundleCmdError> {
    execute_command_with_timeout(zoneadm_list(), DEFAULT_TIMEOUT).await
}

/// Retrieve various `ipadm` command output for the system.
pub async fn ipadm_info(
) -> Vec<Result<SupportBundleCmdOutput, SupportBundleCmdError>> {
    [ipadm_show_interface(), ipadm_show_addr(), ipadm_show_prop()]
        .into_iter()
        .map(|c| async move {
            execute_command_with_timeout(c, DEFAULT_TIMEOUT).await
        })
        .collect::<FuturesUnordered<_>>()
        .collect::<Vec<Result<SupportBundleCmdOutput, SupportBundleCmdError>>>()
        .await
}

/// Retrieve various `dladm` command output for the system.
pub async fn dladm_info(
) -> Vec<Result<SupportBundleCmdOutput, SupportBundleCmdError>> {
    [
        dladm_show_phys(),
        dladm_show_ether(),
        dladm_show_link(),
        dladm_show_vnic(),
        dladm_show_linkprop(),
    ]
        .into_iter()
        .map(|c| async move {
            execute_command_with_timeout(c, DEFAULT_TIMEOUT).await
        })
        .collect::<FuturesUnordered<_>>()
        .collect::<Vec<Result<SupportBundleCmdOutput, SupportBundleCmdError>>>()
        .await
}

/// Retrieve pargs output for all found Oxide processes.
#[cfg(target_os = "illumos")]
pub async fn pargs_oxide_processes(
) -> Vec<Result<SupportBundleCmdOutput, SupportBundleCmdError>> {
    contract::find_oxide_pids()
        .unwrap()
        .iter()
        .map(|pid| pargs_process(*pid))
        .map(|c| async move {
            execute_command_with_timeout(c, DEFAULT_TIMEOUT).await
        })
        .collect::<FuturesUnordered<_>>()
        .collect::<Vec<Result<SupportBundleCmdOutput, SupportBundleCmdError>>>()
        .await
}

#[cfg(not(target_os = "illumos"))]
pub async fn pargs_oxide_processes(
) -> Vec<Result<SupportBundleCmdOutput, SupportBundleCmdError>> {
    let mut command = Command::new("echo");
    command.arg("cannot get pargs on non illumos platforms");
    vec![execute_command_with_timeout(command, DEFAULT_TIMEOUT).await]
}

/// Retrieve pstack output for all found Oxide processes.
#[cfg(target_os = "illumos")]
pub async fn pstack_oxide_processes(
) -> Vec<Result<SupportBundleCmdOutput, SupportBundleCmdError>> {
    contract::find_oxide_pids()
        .unwrap()
        .iter()
        .map(|pid| pstack_process(*pid))
        .map(|c| async move {
            execute_command_with_timeout(c, DEFAULT_TIMEOUT).await
        })
        .collect::<FuturesUnordered<_>>()
        .collect::<Vec<Result<SupportBundleCmdOutput, SupportBundleCmdError>>>()
        .await
}

#[cfg(not(target_os = "illumos"))]
pub async fn pstack_oxide_processes(
) -> Vec<Result<SupportBundleCmdOutput, SupportBundleCmdError>> {
    let mut command = Command::new("echo");
    command.arg("cannot get pargs on non illumos platforms");
    vec![execute_command_with_timeout(command, DEFAULT_TIMEOUT).await]
}
