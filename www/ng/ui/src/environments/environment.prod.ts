export const environment = {
  production: true,

  app_name: "idEAS",

  session_token_key: "token",

  api_url_base: "http://localhost:8081",

  api_sign_up: "/auth/sign-up",
  api_sign_in: "/auth/sign-in",
  
  api_register_start: "/auth/register",
  api_register_complete: "/auth/register/complete",
  api_registration_info: "/auth/register/info",

  api_user_current: "/user/current",
  api_user_client_add: "/user/client/add",

  api_country_fetch: "/countries/fetch",
  api_currency_fetch: "/currencies/fetch",

  api_clients_fetch: "/clients/fetch",
  api_clients_add: "/clients/add",
  api_clients_get: "/clients/get",
  api_clients_update: "/clients/update",

  api_role_add: "/roles/add",
  api_role_update: "/roles/update"
};
