import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { ApiResponse } from 'src/app/classes/api-response';
import { environment } from 'src/environments/environment.prod';

import { v4 as uuidv4 } from 'uuid';


@Injectable({
  providedIn: 'root'
})
export class ProfileService {

  constructor(
    private http: HttpClient
  ) { }

  get(): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_user_profile_get,
      {}
    );
  }

  update(
    people_id: string,
    given_name: string,
    middle_name: string,
    family_name: string,
    prefix: string,
    suffix: string
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_user_profile_update,
      {
        people_id: people_id == '' ? uuidv4() : people_id,
        given_name: given_name,
        middle_name: middle_name,
        family_name: family_name,
        prefix: prefix,
        suffix: suffix
      }
    );
  }

  set_password(
    new_password: string
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_user_profile_password_set,
      {
        new_password: new_password
      }
    );
  }
}
