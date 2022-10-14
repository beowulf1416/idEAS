import { HttpClient, HttpResponse } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { catchError, finalize, map, Observable, of, tap } from 'rxjs';
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
      },
      {
        observe: "response"
      }
    ).pipe(
      map(o => {
        return (o.body as ApiResponse);
      }),
      catchError((e, caught) => {
        console.debug("catchError", e);
        return of({
          success: false,
          message: "error",
          data: null
        });
      })
    );
  }
}
