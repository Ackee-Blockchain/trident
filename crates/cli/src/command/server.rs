use anyhow::Error;
use fehler::throws;
use trident_client::___private::DashboardServer;

#[throws]
pub(crate) async fn server(directory: String, port: u16, host: String) {
    let server = DashboardServer::new(directory, host, port);
    server.start().await?;
}
