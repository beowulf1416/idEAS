import { HttpClient } from '@angular/common/http';
import { Injectable, OnInit } from '@angular/core';
import { BehaviorSubject, catchError, filter, map, Observable } from 'rxjs';
import { environment } from 'src/environments/environment';
import { ApiResponse } from '../classes/api-response';
import { User } from '../classes/user';

@Injectable({
  providedIn: 'root'
})
export class UserService implements OnInit {

  // user$: Observable<User>;

  user$ = new BehaviorSubject<User>(new User('', '', '', [], []));

  constructor(
    private http: HttpClient
  ) {
    console.log("UserService::constructor()");
    this.update();
  }

  ngOnInit(): void {
  }

  get_user$(): Observable<User> {
    return this.user$.asObservable();
  }

  update() {
    this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_user_current,
      {}
    ).subscribe(r => {
      if (r.success) {
        console.debug("UserService::update()", r);
        const u = (r.data as { user: {
          name: string,
          email: string,
          client: string,
          clients: Array<string>,
          permissions: Array<string>
        } }).user;
        this.user$.next(new User(
          u.name,
          u.email,
          u.client,
          u.clients,
          u.permissions
        ));   
      }
    });
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
