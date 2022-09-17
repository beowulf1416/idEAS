import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { environment } from 'src/environments/environment';
import { ApiResponse } from '../classes/api-response';

@Injectable({
  providedIn: 'root'
})
export class AuthService {

  constructor(
    private http: HttpClient
  ) { }

  register(
    email: string
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_base_url + environment.api_user_register,
      {
        email: email
      }
    );
  }
}
