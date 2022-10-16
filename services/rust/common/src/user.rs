use crate::client::Client;

#[derive(Debug, Clone)]
pub struct User {
    id: Option<uuid::Uuid>,
    active: bool,
    email: String,

    clients: Option<Vec<Client>>,
    // current client
    client_id: Option<uuid::Uuid>
}

impl User {

    pub fn new(
        id: Option<uuid::Uuid>,
        active: bool,
        email: String
    ) -> Self {
        return Self {
            id: id,
            active: active,
            email: email,
            clients: None,
            client_id: None
        };
    }

    pub fn is_authenticated(
        &self
    ) -> bool {
        return self.id.is_none();
    }

    pub fn id(&self) -> Option<uuid::Uuid> {
        return self.id;
    }

    pub fn active(&self) -> bool {
        return self.active;
    }

    pub fn email(&self) -> String {
        return format!("{}", self.email);
    }

    pub fn set_clients(
        &mut self,
        clients: Vec<Client>
    ) {
        self.clients = Some(clients);
    }

    pub fn get_clients(
        &self
    ) -> Option<Vec<Client>> {
        return self.clients.clone();
    }

    pub fn set_client(
        &mut self,
        client_id: uuid::Uuid
    ) {
        self.client_id = Some(client_id);
    }

    pub fn get_client(
        &self
    ) -> Option<uuid::Uuid> {
        return self.client_id.clone();
    }
}