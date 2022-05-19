// This file can be replaced during build by using the `fileReplacements` array.
// `ng build --prod` replaces `environment.ts` with `environment.prod.ts`.
// The list of file replacements can be found in `angular.json`.

export const environment = {
  production: false,

  session_token_key: "token",

  path_tenant_select: '/tenant/select',
  path_dashboard: '/dashboard',

  api_tenant_select: '/user/tenant/select',

  api_base_url: 'http://erp.com:8081',
  api_user_signup: '/user/signup',
  api_user_signin: '/user/signin',
  api_user_current: '/user/current',
  api_user_password: '/user/password',
  api_user_tenants: '/user/tenants',
  // api_dashboard: '/dashboard'
};

/*
 * For easier debugging in development mode, you can import the following file
 * to ignore zone related error stack frames such as `zone.run`, `zoneDelegate.invokeTask`.
 *
 * This import should be commented out in production mode because it will have a negative impact
 * on performance if an error is thrown.
 */
// import 'zone.js/plugins/zone-error';  // Included with Angular CLI.
