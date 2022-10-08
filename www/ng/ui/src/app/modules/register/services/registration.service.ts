import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { ApiResponse } from 'src/app/classes/api-response';
import { environment } from 'src/environments/environment';

@Injectable({
  providedIn: 'root'
})
export class RegistrationService {

  constructor(
    private http: HttpClient
  ) { }

  register(
    register_id: string,
    email: string
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_register_start,
      {
        register_id: register_id,
        email: email
      }
    );
  }

  complete(
    register_id: string,
    prefix: string,
    suffix: string,
    given_name: string,
    middle_name: string,
    family_name: string
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_register_complete,
      {
        register_id: register_id,
        prefix: prefix,
        suffix: suffix,
        given_name: given_name,
        middle_name: middle_name,
        family_name: family_name
      }
    );
  }

  get(
    token: string
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_registration_info,
      {
        token: token
      }
    );
  }
}
