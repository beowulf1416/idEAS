import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { from, Observable, of } from 'rxjs';
import { map } from 'rxjs/operators';

import { environment } from 'src/environments/environment';

export interface ApiResponse {
  status: string,
  message: string,
  data: any
}


@Injectable({
  providedIn: 'root'
})
export class UserService {

  permissions: Array<string> = [];

  constructor(
    private http: HttpClient
  ) { }

  is_signed_in(): boolean {
    return sessionStorage.getItem(environment.session_token_key) != null;
  }

  private get_current_user() {
    console.log("UserService::get_current_user()");
    return this.http.post<ApiResponse>(
      environment.api_base_url + environment.api_user_current,
      {}
    );
  }

  current_user() {
    console.log("UserService::current_user()");
    return this.get_current_user();
  }

  get_permissions(): Observable<Array<string>> {
    console.log("UserService::permissions()");

    return this.get_current_user().pipe(
      map((r: ApiResponse) => {
        if (r.status == "success") {
          this.permissions = r.data?.permissions;
          return r.data?.permissions;
        } else {
          return [];
        }
      })
    );
  }
}
