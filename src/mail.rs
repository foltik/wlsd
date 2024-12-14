use anyhow::Result;
use lettre::{
    message::{header::ContentType, Mailbox, MessageBuilder},
    Message, SmtpTransport, Transport,
};

use crate::MailConfig;

#[derive(Clone)]
pub struct Mail {
    addr: String,
    from: Mailbox,
}

impl Mail {
    pub async fn connect(config: MailConfig) -> Result<Self> {
        // we need this for smtps
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
        Ok(Self { addr: config.addr, from: config.from })
    }

    pub fn builder(&self) -> MessageBuilder {
        Message::builder().from(self.from.clone()).header(ContentType::TEXT_PLAIN)
    }

    pub async fn send(&self, message: Message) -> Result<()> {
        let transport = SmtpTransport::from_url(&self.addr)?.build();
        transport.send(&message)?;
        Ok(())
    }
}

// state
//     .mail
//     .send(
//         Message::builder()
//             .from("Radio WLSD <wlsd@foltz.io>".parse()?)
//             .to("Jack Foltz <jack@foltz.io>".parse()?)
//             .subject("Hello, world!")
//             .header(ContentType::TEXT_PLAIN)
//             .body("Be happy!".to_string())?,
//     )
//     .await;
