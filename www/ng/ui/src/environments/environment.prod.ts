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
  api_user_client_set: "/user/client/set",
  api_user_client_add: "/user/client/add",
  api_user_profile_get: "/user/profile/get",
  api_user_profile_update: "/user/profile/update",
  api_user_profile_password_set: "/user/profile/pw",
  api_users_fetch: "/admin/users/fetch",

  api_user_add: "/admin/user/add",
  api_user_get: "/admin/user/get",
  api_user_update: "/admin/user/update",

  api_country_fetch: "/countries/fetch",
  api_currency_fetch: "/currencies/fetch",

  api_clients_fetch: "/clients/fetch",
  api_clients_add: "/clients/add",
  api_clients_get: "/clients/get",
  api_clients_update: "/clients/update",
  api_client_members_fetch: "/clients/members",
  api_client_members_invite: "/clients/members/invite",

  api_role_add: "/roles/add",
  api_role_update: "/roles/update",
  api_role_get: "/roles/get",
  api_roles_fetch: "/roles/fetch",

  api_permissions_assigned: "/permissions/assigned",
  api_permissions_not_assigned: "/permissions/not-assigned"
};
