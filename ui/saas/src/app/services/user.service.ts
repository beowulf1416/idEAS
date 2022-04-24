import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class UserService {

  constructor() { }

  is_signed_in(): boolean {
    return sessionStorage.getItem("token") != null;
  }
}
