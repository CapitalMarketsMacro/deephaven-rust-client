//! Port of the console-script path: `Client`/`TableHandleManager.RunScript`
//! (`Dh_NetClient`). Starts a script session and runs code, which is how named
//! global tables (scope variables) are created on the server.

use crate::auth::Server;
use crate::error::{Error, Result};
use crate::proto::grpc::Ticket;
use crate::proto::script::console_service_client::ConsoleServiceClient;
use crate::proto::script::{ExecuteCommandRequest, StartConsoleRequest};

/// A server-side script session. Scripts run here bind variables into the
/// global query scope, reachable later by name (`s/<name>`).
pub struct Console {
    console_id: Vec<u8>,
}

impl Console {
    /// Start a console of the given session type (e.g. `"python"`). Port of the
    /// `StartConsole` call.
    pub async fn start(server: &Server, session_type: &str) -> Result<Console> {
        let result_id = server.new_export_ticket();
        let request = StartConsoleRequest {
            result_id: Some(Ticket { ticket: result_id.clone() }),
            session_type: session_type.to_string(),
        };

        let mut console = ConsoleServiceClient::new(server.channel());
        let response = console
            .start_console(server.authorize(request)?)
            .await?
            .into_inner();

        let console_id = response.result_id.map(|t| t.ticket).unwrap_or(result_id);
        Ok(Console { console_id })
    }

    /// Execute a script in this session. Port of `TableHandleManager.RunScript`.
    pub async fn run_script(&self, server: &Server, code: &str) -> Result<()> {
        let request = ExecuteCommandRequest {
            console_id: Some(Ticket { ticket: self.console_id.clone() }),
            code: code.to_string(),
            systemic: None,
        };

        let mut console = ConsoleServiceClient::new(server.channel());
        let response = console
            .execute_command(server.authorize(request)?)
            .await?
            .into_inner();

        if !response.error_message.is_empty() {
            return Err(Error::Server(response.error_message));
        }
        Ok(())
    }
}
