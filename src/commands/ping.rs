use crate::{commands::Command, utils};
use async_trait::async_trait;
use psutil::process::Process;
use std::{process, time::Instant};
use twitch_irc::{
    client::TwitchIRCClient, login::StaticLoginCredentials, message::PrivmsgMessage, Error,
    SecureTCPTransport,
};

pub struct Ping {
    start_time: Instant,
    process: Process,
}

impl Ping {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            process: Process::new(process::id()).unwrap_or_else(|e| {
                eprintln!("Failed to create new process: {e}");
                process::exit(1);
            }),
        }
    }
}

#[async_trait]
impl Command for Ping {
    fn triggers(&self) -> Vec<String> {
        vec!["ping".to_owned(), "pong".to_string(), "pang".to_string()]
    }

    async fn run(
        &self,
        client: &TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
        msg: PrivmsgMessage,
        _args: Vec<String>,
    ) -> Result<(), Error<SecureTCPTransport, StaticLoginCredentials>> {
        let uptime = utils::format_duration(self.start_time.elapsed());
        let memory = self
            .process
            .memory_info()
            .map(|m| format!("{:.0}mb", utils::bytes_to_mb(m.rss())))
            .unwrap_or_else(|e| format!("{e}"));

        client
            .say(
                msg.channel_login,
                format!("forsen uptime: {} ⦁ mem: {}", uptime, memory),
            )
            .await?;

        Ok(())
    }
}
