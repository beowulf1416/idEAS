import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { ApiResponse } from 'src/app/classes/api-response';
import { environment } from 'src/environments/environment.prod';

@Injectable({
  providedIn: 'root'
})
export class UsersService {

  constructor(
    private http: HttpClient
  ) { }

  fetch(
    filter: string,
    items: number,
    page: number
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_users_fetch,
      {
        filter: filter,
        items: items,
        page: page
      }
    );
  }

  add(
    user_id: string,
    email: string,
    password: string
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_user_add,
      {
        user_id: user_id,
        email: email,
        password: password
      }
    );
  }

  update(
    user_id: string,
    email: string,
    password: string
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_user_update,
      {
        user_id: user_id,
        email: email,
        password: password
      }
    );
  }

  get(
    user_id: string
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_user_get,
      {
        user_id: user_id
      }
    );
  }
}
