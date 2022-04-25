import { HttpClient, HttpErrorResponse, HttpResponse } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable, throwError } from 'rxjs';
import { catchError, tap } from 'rxjs/operators';
import { UserService } from 'src/app/services/user.service';

import { environment } from 'src/environments/environment';


export interface ApiResponse {
  status: string,
  message: string,
  data: any
}

@Injectable({
  providedIn: 'root'
})
export class SigninService {

  constructor(
    private http: HttpClient,
    private user: UserService
  ) { }

  signIn(
    email: string,
    password: string
  ): Observable<HttpResponse<ApiResponse>> {
    return this.http.post<ApiResponse>(
      environment.api_base_url + environment.api_user_signin,
      {
        email: email,
        password: password
      },
      {
        observe: 'response'
      }
    ).pipe(
      tap(o => {
        if (o.headers.has("authorization")){
          const authorization = o.headers.get("authorization");
          if (authorization != null) {
            const token = authorization?.replace("Bearer", "");
            sessionStorage.setItem(environment.session_token_key, token);

            // retrieve roles
            this.user.current_user().subscribe(r => {
              console.log("SignService::signIn()");
              console.log(r);
            });
          }
        }
      }),
      catchError(err => this.handleError(err))
    )
  }

  handleError(error: HttpErrorResponse) {
    if (error.status === 0) {
      return throwError({
        status: "error",
        message: "client side error",
        data: {}
      });
    } else {
      return throwError({
        status: "error",
        message: "backend error",
        data: {}
      });
    }
  }
}
