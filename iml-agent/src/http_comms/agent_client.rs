// Copyright (c) 2020 DDN. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::{agent_error::ImlAgentError, http_comms::crypto_client, server_properties};
use futures::{Future, TryFutureExt};
use iml_wire_types::{Id, PluginName};
use reqwest::Client;
use std::convert::Into;
use tracing::{debug, info};

/// A wrapper around `CryptoClient`.
///
/// Provides abstraction for common requests to the manager.
#[derive(Debug, Clone)]
pub struct AgentClient {
    start_time: String,
    message_endpoint: url::Url,
    client: Client,
}

impl AgentClient {
    pub fn new(start_time: String, message_endpoint: url::Url, client: Client) -> Self {
        Self {
            start_time,
            message_endpoint,
            client,
        }
    }
    /// Send a request to the manager
    ///
    /// # Arguments
    ///
    /// * `message` - The message to send
    pub fn post(
        &self,
        message: iml_wire_types::Message,
    ) -> impl Future<Output = Result<String, ImlAgentError>> {
        let envelope = iml_wire_types::Envelope::new(
            vec![message],
            self.start_time.clone(),
            server_properties::BOOT_TIME.to_string(),
        );

        crypto_client::post(&self.client, self.message_endpoint.clone(), &envelope)
    }
    /// Send a new session request to the manager
    ///
    /// # Arguments
    ///
    /// * `plugin` - The plugin to initiate a session over
    pub fn create_session(
        &self,
        plugin: iml_wire_types::PluginName,
    ) -> impl Future<Output = Result<(), ImlAgentError>> {
        info!("Requesting new session for: {:?}.", plugin);

        let m: iml_wire_types::Message = iml_wire_types::Message::SessionCreateRequest {
            fqdn: iml_wire_types::Fqdn(server_properties::FQDN.to_string()),
            plugin,
        };

        self.post(m).map_ok(drop)
    }
    /// Send data to the manager
    ///
    /// # Arguments
    /// * `info` - Bundle of session info
    /// * `output` - The data to send
    pub fn send_data(
        &self,
        id: Id,
        name: PluginName,
        seq: u64,
        body: impl serde::Serialize + std::fmt::Debug,
    ) -> impl Future<Output = Result<(), ImlAgentError>> + '_ {
        debug!("Sending session data for {:?}({:?}): {:?}", name, id, body);

        let value = serde_json::to_value(body);

        async move {
            let value = value?;

            let m = iml_wire_types::Message::Data {
                fqdn: iml_wire_types::Fqdn(server_properties::FQDN.to_string()),
                plugin: name,
                session_id: id,
                session_seq: seq,
                body: value,
            };

            self.post(m).await?;

            Ok(())
        }
    }
    /// Get data from the manager
    ///
    /// # Arguments
    ///
    pub fn get(
        &self,
    ) -> impl Future<Output = Result<iml_wire_types::ManagerMessages, ImlAgentError>> {
        let get_params: Vec<(String, String)> = vec![
            (
                "server_boot_time".into(),
                server_properties::BOOT_TIME.to_string(),
            ),
            ("client_start_time".into(), self.start_time.clone()),
        ];

        debug!("Sending get {:?}", get_params);

        crypto_client::get_buffered(&self.client, self.message_endpoint.clone(), &get_params)
            .and_then(|x| async move { serde_json::from_str(&x).map_err(Into::into) })
    }
}
