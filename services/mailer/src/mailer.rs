#[cfg(test)]
mod tests {

    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{ Message, SmtpTransport, Transport };

    #[test]
    fn test_send() {
        let email = Message::builder()
            .from("ferdinand <beowulf1416@gmail.com>".parse().unwrap())
            // .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
            .to("ferdinand <ferdinand@marginfuel.com>".parse().unwrap())
            .subject("Happy new year")
            .body(String::from("Be happy!"))
            .unwrap();

        let creds = Credentials::new("beowulf1416@gmail.com".to_string(), "sOo8DKZe8129".to_string());

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }
}
