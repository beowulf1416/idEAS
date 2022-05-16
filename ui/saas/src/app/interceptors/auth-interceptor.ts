import { Injectable } from '@angular/core';
import {
  HttpEvent, HttpInterceptor, HttpHandler, HttpRequest, HttpEventType
} from '@angular/common/http';

import { Observable } from 'rxjs';
import { environment } from 'src/environments/environment';


@Injectable()
export class AuthInterceptor implements HttpInterceptor {

    intercept(req: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {
        console.log("AuthInterceptor::intercept()");
        const token = sessionStorage.getItem(environment.session_token_key);

        if (token != null) {
            console.log("AuthInterceptor::intercept() Adding auth token");
            const authReq = req.clone({
                setHeaders: { Authorization: `Bearer ${token}` }
            });

            return next.handle(authReq);
        } else {
            return next.handle(req);
        }
    }
}