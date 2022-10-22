// export interface User {
//     email: string,
//     name: string
// }

import { environment } from "src/environments/environment";

export class User {

    constructor(
        private email: string,
        private name: string
    ){}

    get email_address(): string {
        return this.email;
    }

    get fullname(): string {
        return this.name;
    }

    get is_signed_in(): boolean {
        // console.debug("User::is_signed_in()", (sessionStorage.getItem(environment.session_token_key) || '') != ''
        //     && this.email != '');
        return (sessionStorage.getItem(environment.session_token_key) || '') != ''
            && this.email != '';
    }
}
