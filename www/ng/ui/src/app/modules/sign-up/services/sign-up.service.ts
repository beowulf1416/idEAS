import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { ApiResponse } from 'src/app/classes/api-response';
import { environment } from 'src/environments/environment';

import { v4 as uuidv4 } from 'uuid';


@Injectable({
  providedIn: 'root'
})
export class SignUpService {

  constructor(
    private http: HttpClient
  ) { }

  sign_up(
    email: string,
    password: string
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_sign_up,
      {
        id: uuidv4(),
        email: email,
        password: password
      }
    )
  }
}
