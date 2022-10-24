// This file can be replaced during build by using the `fileReplacements` array.
// `ng build` replaces `environment.ts` with `environment.prod.ts`.
// The list of file replacements can be found in `angular.json`.

export const environment = {
  production: false,

  app_name: "idEAS",

  session_token_key: "token",

  api_url_base: "http://localhost:8081",

  api_sign_up: "/auth/sign-up",
  api_sign_in: "/auth/sign-in",
  
  api_register_start: "/auth/register",
  api_register_complete: "/auth/register/complete",
  api_registration_info: "/auth/register/info",

  api_user_current: "/user/current",

  api_country_fetch: "/countries/fetch",
  api_currency_fetch: "/currencies/fetch",

  api_clients_fetch: "/clients/fetch",
  api_clients_add: "/clients/add",
  api_clients_get: "/clients/get",
  api_clients_update: "/clients/update"
};

/*
 * For easier debugging in development mode, you can import the following file
 * to ignore zone related error stack frames such as `zone.run`, `zoneDelegate.invokeTask`.
 *
 * This import should be commented out in production mode because it will have a negative impact
 * on performance if an error is thrown.
 */
// import 'zone.js/plugins/zone-error';  // Included with Angular CLI.
