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

@Injectable()
export class AuthInterceptor implements HttpInterceptor {

  constructor() {}

  intercept(request: HttpRequest<unknown>, next: HttpHandler): Observable<HttpEvent<unknown>> {
    console.log("AuthInterceptor::intercept()");
    const token = sessionStorage.getItem(environment.session_token_key);

    const authReq = request.clone({
      setHeaders: { Authorization: `Bearer ${token}` }
    });
  
    return next.handle(authReq)
      .pipe(
        tap({
          next: (e) => {
            console.log("AuthInterceptor::intercept()", e);
            console.log(e instanceof HttpResponse);
          }
        })
      );


    // if (token != null) {
    //     console.log("AuthInterceptor::intercept() Adding auth token");
    //     const authReq = request.clone({
    //         setHeaders: { Authorization: `Bearer ${token}` }
    //     });

    //     return next.handle(authReq);
    // } else {
    //     return next.handle(request);
    // }
  }
}
