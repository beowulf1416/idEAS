// export interface User {
//     email: string,
//     name: string
// }

import { environment } from "src/environments/environment";

export class User {

    constructor(
        private _email: string,
        private _name: string,
        private _client: string,
        private _clients: Array<{ id: string, name: string }>,
        private _permissions: Array<string>
    ){}

    get email(): string {
        return this._email;
    }

    get name(): string {
        return this._name;
    }

    get is_signed_in(): boolean {
        // console.debug("User::is_signed_in()", (sessionStorage.getItem(environment.session_token_key) || '') != ''
        //     && this.email != '');
        return (sessionStorage.getItem(environment.session_token_key) || '') != ''
            && this._email != '';
    }

    get client(): string {
        return this._client || '' == '' ? 'default' : this._client;
    }

    get clients(): Array<{ id: string, name: string }> {
        return this._clients;
    }

    get permissions(): Array<string> {
        return this._permissions;
    }
}
