import { Injectable } from '@angular/core';
import {
  HttpEvent, HttpInterceptor, HttpHandler, HttpRequest, HttpEventType
} from '@angular/common/http';

import { Observable } from 'rxjs';


@Injectable()
export class AuthInterceptor implements HttpInterceptor {

    intercept(req: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {
        console.log("AuthInterceptor::intercept()");
        console.log(req);

        const authReq = req.clone({
            setHeaders: { Authorization: "sample" }
        });

        return next.handle(authReq);
    }
}