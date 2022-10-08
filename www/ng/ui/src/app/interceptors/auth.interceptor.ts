import { Injectable } from '@angular/core';
import {
  HttpRequest,
  HttpHandler,
  HttpEvent,
  HttpInterceptor
} from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from 'src/environments/environment';

@Injectable()
export class AuthInterceptor implements HttpInterceptor {

  constructor() {}

  intercept(request: HttpRequest<unknown>, next: HttpHandler): Observable<HttpEvent<unknown>> {
    console.log("AuthInterceptor::intercept()");
    const token = sessionStorage.getItem(environment.session_token_key);

    if (token != null) {
        console.log("AuthInterceptor::intercept() Adding auth token");
        const authReq = request.clone({
            setHeaders: { Authorization: `Bearer ${token}` }
        });

        return next.handle(authReq);
    } else {
        return next.handle(request);
    }
  }
}
