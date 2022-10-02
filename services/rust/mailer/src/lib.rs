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
    transport::smtp::authentication::Credentials,
    message::{
        header,
        MultiPart,
        SinglePart
    }
};


#[derive(Debug)]
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
        match Message::builder()
            .from(from.parse().unwrap())
            .to(to.parse().unwrap())
            .reply_to(to.parse().unwrap())
            .subject(subject)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(String::from(body))
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(String::from(body))
                    )
            ) {
            // .body(String::from(body)) {
            Err(e) => {
                error!("error while building message: {:?}", e);
                return Err(MailerError::SendError);
            }
            Ok(email) => {
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
                    error!("transport is None");
                    return Err(MailerError::SendError);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use std::sync::Once;
    static INIT: Once = Once::new();

    use super::*;


    fn initialize() {
        INIT.call_once( || {
            env_logger::init();
        });
    }

    #[test]
    fn test_send() {
        initialize();

        let mut mailer = Mailer::new(
            "smtp.gmail.com",
            "beowulf1416@gmail.com",
            // if using smtp gmail, the password below should be generated
            // using App Passwords in Google Account Settings
            "<google_app_password>"
        );

        if let Err(e) = mailer.connect() {
            error!("error: {:?}", e);
            assert!(false);
        } else {
            if let Err(e) = mailer.send(
                "beowulf1416@gmail.com",
                "ferdinand@marginfuel.com",
                "testing",
                "this is a test"
            ) {
                error!("error: {:?}", e);
                assert!(false);
            } else {
                assert!(true);
            }
        }
    }
}