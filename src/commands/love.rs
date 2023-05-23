use crate::commands::Command;
use async_trait::async_trait;
use rand::Rng;
use twitch_irc::{
    client::TwitchIRCClient, login::StaticLoginCredentials, message::PrivmsgMessage, Error,
    SecureTCPTransport,
};

pub struct Love {}

impl Love {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Command for Love {
    fn triggers(&self) -> Vec<String> {
        vec!["love".to_string()]
    }

    async fn run(
        &self,
        client: &TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
        msg: PrivmsgMessage,
        args: Vec<String>,
    ) -> Result<(), Error<SecureTCPTransport, StaticLoginCredentials>> {
        if args.len() < 2 {
            return Ok(());
        }

        let random_number = rand::thread_rng().gen_range(1..=100);
        let message = if args.len() >= 3 {
            format!(
                "Love between {} and {} is {}% PogChamp ❤",
                args[1], args[2], random_number
            )
        } else {
            format!(
                "Love between {} and {} is {}% PogChamp ❤",
                msg.sender.name, args[1], random_number
            )
        };

        client.say(msg.channel_login, message).await?;

        Ok(())
    }
}
