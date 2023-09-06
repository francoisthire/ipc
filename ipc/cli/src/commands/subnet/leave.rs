// Copyright 2022-2023 Protocol Labs
// SPDX-License-Identifier: MIT
//! Leave subnet cli command handler.

use async_trait::async_trait;
use clap::Args;
use std::fmt::Debug;

use crate::commands::get_ipc_agent_url;
use crate::sdk::IpcAgentClient;
use crate::server::leave::LeaveSubnetParams;
use crate::{CommandLineHandler, GlobalArguments};

/// The command to leave a new subnet.
pub struct LeaveSubnet;

#[async_trait]
impl CommandLineHandler for LeaveSubnet {
    type Arguments = LeaveSubnetArgs;

    async fn handle(global: &GlobalArguments, arguments: &Self::Arguments) -> anyhow::Result<()> {
        log::debug!("leave subnet with args: {:?}", arguments);

        let params = LeaveSubnetParams {
            subnet: arguments.subnet.clone(),
            from: arguments.from.clone(),
        };

        let url = get_ipc_agent_url(&arguments.ipc_agent_url, global)?;

        let client = IpcAgentClient::default_from_url(url);
        client.leave_subnet(params).await?;

        log::info!("left subnet: {:}", arguments.subnet);

        Ok(())
    }
}

#[derive(Debug, Args)]
#[command(name = "leave", about = "Leaving a subnet")]
pub struct LeaveSubnetArgs {
    #[arg(long, short, help = "The JSON RPC server url for ipc agent")]
    pub ipc_agent_url: Option<String>,
    #[arg(long, short, help = "The address that leaves the subnet")]
    pub from: Option<String>,
    #[arg(long, short, help = "The subnet to leave")]
    pub subnet: String,
}
