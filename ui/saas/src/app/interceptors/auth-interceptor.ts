import { Injectable } from '@angular/core';
import {
  HttpEvent, HttpInterceptor, HttpHandler, HttpRequest, HttpEventType
} from '@angular/common/http';

import { Observable } from 'rxjs';


@Injectable()
export class AuthInterceptor implements HttpInterceptor {

    intercept(req: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {
        console.log("AuthInterceptor::intercept()");
        // console.log(req);

        const token = sessionStorage.getItem("token");

        if (token != null) {
            const authReq = req.clone({
                setHeaders: { Authorization: `Bearer ${token}` }
            });

            return next.handle(authReq);
        } else {
            return next.handle(req);
        }
    }
}