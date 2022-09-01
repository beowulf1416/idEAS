use log::{
    info,
    error,
    debug
};

// use lettre::transport::smtp::authentication::Credentials;
use lettre::{
    Message, 
    SmtpTransport, 
    Transport,
    transport::smtp::authentication::Credentials
};


pub enum MailerError {
    ConnectionError,
    SendError
}


pub struct Mailer {
    relay_host: String,
    credentials: Credentials,
    transport: Option<SmtpTransport> 
}


impl Mailer {

    pub fn new(
        smtp_host: &str,
        smtp_user: &str,
        smtp_password: &str
    ) -> Self {
        let creds = Credentials::new(
            String::from(smtp_user),
            String::from(smtp_password) 
        );

        return Self {
            relay_host: String::from(smtp_host),
            credentials: creds,
            transport: None
        };
    }


    pub fn connect(&mut self) -> Result<(), MailerError> {
        let mailer = SmtpTransport::relay(&self.relay_host)
            .unwrap()
            .credentials(self.credentials.clone())
            .build();

        self.transport = Some(mailer);

        return Ok(());
    }

    pub fn send(
        &self,
        from: &str,
        to: &str,
        subject: &str,
        body: &str
    ) -> Result<(), MailerError> {
        let email = Message::builder()
            // .from("NoBody <nobody@domain.tld>".parse().unwrap())
            // .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
            // .to("Hei <hei@domain.tld>".parse().unwrap())
            // .subject("Happy new year")
            // .body(String::from("Be happy!"))
            .from(from.parse().unwrap())
            .reply_to(to.parse().unwrap())
            .subject(subject)
            .body(String::from(body))
            .unwrap();

        if let Some(transport) = &self.transport {
            match transport.send(&email) {
                Err(e) => {
                    error!("unable to send email: {:?}", e);
                    return Err(MailerError::SendError);
                }
                Ok(_) => {
                    info!("email sent");
                    return Ok(());
                }
            }
        } else {
            return Err(MailerError::SendError);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}