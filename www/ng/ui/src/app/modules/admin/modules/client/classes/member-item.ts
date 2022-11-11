export class MemberItem {
    constructor(
        private _id: string,
        private _active: boolean,
        private _email: string
    ) {}

    get id(): string {
        return this._id;
    }

    get active(): boolean {
        return this._active;
    }

    get email(): string {
        return this._email;
    }
}
