use crate::{
    commands::{love::Love, ping::Ping, Command},
    config::Config,
};
use std::{collections::HashMap, error, time::Instant};
use tokio::sync::mpsc::UnboundedReceiver;
use twitch_irc::{
    login::StaticLoginCredentials, message::PrivmsgMessage, message::ServerMessage, validate,
    ClientConfig, SecureTCPTransport, TwitchIRCClient,
};

pub struct Handler {
    cfg: Config,
    client: TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
    messages: UnboundedReceiver<ServerMessage>,
    commands: Vec<Box<dyn Command>>,
    execution_log: HashMap<String, Instant>,
}

impl Handler {
    pub fn new(cfg: Config) -> Self {
        let client_config = ClientConfig::new_simple(StaticLoginCredentials::new(
            cfg.twitch.login.clone(),
            cfg.twitch.oauth.clone(),
        ));

        let (incoming_messages, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(client_config);

        Self {
            cfg,
            client,
            messages: incoming_messages,
            commands: vec![
                Box::new(Ping::new()) as Box<dyn Command>,
                Box::new(Love::new()) as Box<dyn Command>,
            ],
            execution_log: HashMap::new(),
        }
    }

    async fn handle_command(
        &mut self,
        msg: PrivmsgMessage,
    ) -> Result<(), twitch_irc::Error<SecureTCPTransport, StaticLoginCredentials>> {
        if !msg.message_text.starts_with(&self.cfg.prefix) {
            return Ok(());
        }

        let args: Vec<String> = msg
            .message_text
            .trim()
            .split(' ')
            .map(String::from)
            .skip(1)
            .collect();

        if args.is_empty() {
            return Ok(());
        }

        if let Some(command) = self
            .commands
            .iter()
            .find(|command| command.triggers().iter().any(|trigger| trigger == &args[0]))
        {
            let now = Instant::now();
            let sender_id = msg.sender.id.clone();

            if let Some(last_execution) = self.execution_log.get(&sender_id) {
                if now.duration_since(*last_execution) < command.cooldown() {
                    return Ok(());
                }
            }

            // TODO: use tokio::spawn() somehow, prolly will have to rewrite all of this anyways xD
            command.run(&self.client, msg, args).await?;

            self.execution_log.insert(sender_id, now);
        }

        Ok(())
    }

    // TODO: refactor this, use regex and stuff
    async fn handle_noncommands(
        &self,
        msg: twitch_irc::message::PrivmsgMessage,
    ) -> Result<(), Box<dyn error::Error>> {
        if msg.message_text.starts_with(&self.cfg.prefix) {
            return Ok(());
        }

        if msg.channel_id == "11148817" {
            if msg.sender.id == "477589350" && msg.message_text == "PepeA pajbot" {
                self.client
                    .privmsg(msg.channel_login, "/me PAJAS I'M GONNA SNEEZE".to_string())
                    .await?;

                return Ok(());
            }

            if msg.is_action && msg.sender.id == "82008718" && msg.message_text == "pajaS 🚨 ALERT"
            {
                self.client
                    .privmsg(msg.channel_login, "/me PAJAS 🚨 ACHOOO".to_string())
                    .await?;

                return Ok(());
            }

            return Ok(());
        }

        // felyp
        if msg.channel_id == "162760707"
            && msg.is_action
            && msg.sender.id == "743355647"
            && msg.message_text == "PajaS"
        {
            self.client
                .privmsg(msg.channel_login, "/me PajaS".to_string())
                .await?;

            return Ok(());
        }

        if msg.message_text.contains("forsen") && rand::random() {
            self.client
                .say(msg.channel_login, "forsen".to_string())
                .await?
        }

        Ok(())
    }

    pub async fn join_all_channels(&self) -> Result<(), validate::Error> {
        for channel in self.cfg.channels.iter() {
            self.client.join(channel.login.clone())?
        }

        Ok(())
    }

    pub async fn part_all_channels(&self) -> Result<(), validate::Error> {
        for channel in self.cfg.channels.iter() {
            self.client.join(channel.login.clone())?
        }

        Ok(())
    }

    pub async fn handle_events(mut self) -> Result<(), Box<dyn error::Error>> {
        self.join_all_channels().await?;

        let handle = tokio::spawn(async move {
            while let Some(message) = self.messages.recv().await {
                match message {
                    ServerMessage::Join(msg) => {
                        println!("Joined: {}", msg.channel_login);
                    }

                    ServerMessage::Part(msg) => {
                        println!("Parted: {}", msg.channel_login);
                    }

                    ServerMessage::Privmsg(msg) => {
                        if msg.sender.login == self.cfg.twitch.login {
                            continue;
                        }

                        // TODO: better logging?
                        if let Err(e) = self.handle_command(msg.clone()).await {
                            eprintln!("Error happened while handling command: {e}")
                        };

                        if let Err(e) = self.handle_noncommands(msg.clone()).await {
                            eprintln!("Error happened while handling non-command: {e}")
                        };
                    }

                    _ => (),
                }
            }
        });

        handle.await?;

        Ok(())
    }
}
