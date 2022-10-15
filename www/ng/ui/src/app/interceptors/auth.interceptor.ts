import { Injectable } from '@angular/core';
import {
  HttpRequest,
  HttpHandler,
  HttpEvent,
  HttpInterceptor,
  HttpResponse
} from '@angular/common/http';
import { finalize, Observable, tap } from 'rxjs';
import { environment } from 'src/environments/environment';
import { ApiResponse } from '../classes/api-response';

@Injectable()
export class AuthInterceptor implements HttpInterceptor {

  constructor() {}

  intercept(request: HttpRequest<unknown>, next: HttpHandler): Observable<HttpEvent<unknown>> {
    console.log("AuthInterceptor::intercept()");
    const token = sessionStorage.getItem(environment.session_token_key) || '';

    let authReq = request.clone();
    if (token != '') {
      console.log("adding authorization header");
      authReq = request.clone({
        setHeaders: { Authorization: `Bearer ${token}` }
      });
    }
  
    return next.handle(authReq)
      .pipe(
        tap({
          next: (e) => {
            if (e instanceof HttpResponse) {
              let response = (e as HttpResponse<ApiResponse>);
              if (response.ok && response.headers.has("authorization")) {
                let new_token = response.headers.get("authorization")?.replace("Bearer", "").trim() || '';
                if (token != new_token) {
                  sessionStorage.setItem(environment.session_token_key, new_token);
                }
              }
            } else {
              console.debug("AuthInterceptor::intercept()", e);
            }
          }
        })
      );
  }
}
