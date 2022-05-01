import { HttpClient, HttpErrorResponse } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable, throwError } from 'rxjs';

import { ApiResponse } from 'src/app/classes/api-response';
import { environment } from 'src/environments/environment';

@Injectable({
  providedIn: 'root'
})
export class PasswordService {

  constructor(
    private http: HttpClient
  ) { }

  change(
    new_password: string
  ) {
    return this.http.post<ApiResponse>(
      environment.api_base_url + environment.api_user_password,
      {
        password: new_password
      }
    );
  }

  // handleError(error: HttpErrorResponse) {
  //   if (error.status === 0) {
  //     return throwError({
  //       status: "error",
  //       message: "client side error",
  //       data: {}
  //     });
  //   } else {
  //     return throwError({
  //       status: "error",
  //       message: "backend error",
  //       data: {}
  //     });
  //   }
  // }
}
