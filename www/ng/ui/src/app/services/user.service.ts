import { HttpClient } from '@angular/common/http';
import { Injectable, OnInit } from '@angular/core';
import { catchError, filter, map, Observable } from 'rxjs';
import { environment } from 'src/environments/environment';
import { ApiResponse } from '../classes/api-response';
import { User } from '../classes/user';

@Injectable({
  providedIn: 'root'
})
export class UserService implements OnInit {

  user$: Observable<User>;

  constructor(
    private http: HttpClient
  ) {
    this.user$ = this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_user_current,
      {}
    ).pipe(
      filter((r => r.success == true)),
      map(r => {
        console.debug("r", r);
        const data = (r.data as { user: {
          email: string,
          name: string
        } });
        // return {
        //   email: data?.user?.email,
        //   name: data?.user?.name || ''
        // };
        return new User(
          data?.user?.email,
          data?.user?.name
        );
      }),
      catchError(e => {
        console.debug("catchError", e);
        return new Observable<User>(function(observer) {
          // observer.next({
          //   success: true,
          //   message: "//TODO default",
          //   data: {
          //     user: new User('', ''),
          //     error: e
          //   }
          // });
          observer.next(new User('', ''));
          observer.complete();
        });
      })
    );
  }

  ngOnInit(): void {
  }

  // current(): Observable<ApiResponse> {
  //   console.log("UserService::current()");
  //   return this.http.post<ApiResponse>(
  //     environment.api_url_base + environment.api_user_current,
  //     {}
  //   ).pipe(
  //     catchError(e => {
  //       console.debug("catchError", e);
  //       return new Observable<ApiResponse>(function(observer) {
  //         observer.next({
  //           success: true,
  //           message: "//TODO default",
  //           data: {
  //             user: new User('', ''),
  //             error: e
  //           }
  //         });
  //       });
  //     })
  //   );
  // }

  sign_in(
    email: string,
    pw: string
  ): Observable<ApiResponse> {
    console.log("UserService::sign_in()");
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_sign_in,
      {
        email: email,
        password: pw
      }
    );
  }
}
