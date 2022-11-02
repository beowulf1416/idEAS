import { HttpClient } from '@angular/common/http';
import { Injectable, OnInit } from '@angular/core';
import { BehaviorSubject, catchError, filter, map, Observable, of, tap } from 'rxjs';
import { environment } from 'src/environments/environment';
import { ApiResponse } from '../classes/api-response';
import { MessageType } from '../classes/message-type';
import { User } from '../classes/user';
import { MessageService } from './message.service';

@Injectable({
  providedIn: 'root'
})
export class UserService implements OnInit {

  // user$: Observable<User>;

  user$ = new BehaviorSubject<User>(new User('', '', '', [], []));

  constructor(
    private http: HttpClient,
    private msg_service: MessageService
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
    console.log("UserService::update()");
    if (sessionStorage.getItem(environment.session_token_key) != null) {
      this.http.post<ApiResponse>(
        environment.api_url_base + environment.api_user_current,
        {}
      ).pipe(
        catchError(e => {
          return of({
            success: false,
            message: e.statusText,
            data: null
          });
        })
      ).subscribe(r => {
        if (r.success) {
          const u = (r.data as { user: {
            name: string,
            email: string,
            client: string,
            clients: Array<{ id: string, name: string }>,
            permissions: Array<string>
          } }).user;
          this.user$.next(new User(
            u.email,
            u.name,
            u.client,
            u.clients,
            u.permissions
          ));   
        } else {
          this.msg_service.send(r.message, MessageType.error);
        }
      });
    }
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
    const self = this;
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_sign_in,
      {
        email: email,
        password: pw
      }
    ).pipe(
      tap(r => {
        if (r.success) {
          self.update();
        }
      })
    );
  }

  sign_out() {
    console.log("UserService::sign_out()");
    sessionStorage.removeItem(environment.session_token_key);
  }

  get_profile(): Observable<ApiResponse> {
    console.log("UserService::get_profile()");
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_user_profile_get,
      {}
    );
  }

  set_client(client_id: string) {
    console.log("UserService::set_client()", client_id);
    this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_user_client_set,
      {
        client_id: client_id
      }
    ).subscribe(r => {
      if (r.success) {
        this.msg_service.send(r.message, MessageType.info);
      } else {
        console.error("UserService::set_client()", r);
        this.msg_service.send(r.message, MessageType.error);
      }
    });
  }
}
