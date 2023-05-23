pub mod love;
pub mod ping;

use async_trait::async_trait;
use std::time::Duration;
use twitch_irc::{
    login::StaticLoginCredentials, message::PrivmsgMessage, Error, SecureTCPTransport,
    TwitchIRCClient,
};

#[derive(Debug)]
pub enum Permision {
    Admin,
    Broadcaster,
    Moderator,
    Vip,
    User,
}

#[async_trait]
pub trait Command: Send + Sync {
    fn triggers(&self) -> Vec<String>;

    fn permision(&self) -> Permision {
        Permision::User
    }

    fn cooldown(&self) -> Duration {
        Duration::from_secs(5)
    }

    async fn run(
        &self,
        client: &TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
        msg: PrivmsgMessage,
        agrs: Vec<String>,
    ) -> Result<(), Error<SecureTCPTransport, StaticLoginCredentials>>;
}
