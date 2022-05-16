import { HttpClient, HttpErrorResponse } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable, throwError } from 'rxjs';
import { catchError } from 'rxjs/operators';

import { environment } from 'src/environments/environment';

export interface ApiResponse {
  status: string,
  message: string,
  data: any
}


@Injectable({
  providedIn: 'root'
})
export class TenantService {

  constructor(
    private http: HttpClient
  ) { }

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

  get_tenants(): Observable<ApiResponse> {
    console.log("TenantService::get_tenants()");
    return this.http.post<ApiResponse>(
      environment.api_base_url + environment.api_user_tenants,
      {}
    ).pipe(
      catchError(err => this.handleError(err))
    );
  }
}
