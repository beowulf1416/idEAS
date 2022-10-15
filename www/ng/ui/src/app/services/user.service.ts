import { HttpClient } from '@angular/common/http';
import { Injectable, OnInit } from '@angular/core';
import { filter, map, Observable } from 'rxjs';
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
        const data = (r.data as { user: User });
        return {
          email: data?.user?.email,
          name: data?.user?.name || ''
        };
      })
    );
  }

  ngOnInit(): void {
  }

  current(): Observable<ApiResponse> {
    console.log("UserService::current()");
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_user_current,
      {}
    );
  }

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
