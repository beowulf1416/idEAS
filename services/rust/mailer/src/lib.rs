use log::{
    info,
    error,
    debug
};



pub struct Mailer {

}


impl Mailer {

    pub fn new(
        smtp_host: &str,
        smtp_user: &str,
        smtp_password: &str
    ) -> Self {
        return Self {

        };
    }

    pub fn send(
        &self,
        from: &str,
        to: &str,
        subject: &str,
        body: &str
    ) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;
}