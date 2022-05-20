import { Injectable } from '@angular/core';
import { ActivatedRouteSnapshot, CanActivate, Router, RouterStateSnapshot, UrlTree } from '@angular/router';
import { Observable, of } from 'rxjs';
import { map } from 'rxjs/operators';

import { environment } from 'src/environments/environment';
import { UserService } from '../services/user.service';

@Injectable({
  providedIn: 'root'
})
export class AuthGuard implements CanActivate {

  constructor(
    private router: Router,
    private user: UserService
  ) {}

  canActivate(
    route: ActivatedRouteSnapshot,
    state: RouterStateSnapshot): Observable<boolean | UrlTree> | Promise<boolean | UrlTree> | boolean | UrlTree {

    const token = sessionStorage.getItem(environment.session_token_key);
    if (token == null) {
      return this.router.parseUrl("/user/signin");
    } else {
      // TODO check permissions
      console.log("AuthGuard::canActivate() check permissions: ", route.data?.permission);
      if (route.data?.permission) {
        return this.user.get_permissions().pipe(
          map((r: string[]) => {
            return r.includes(route.data?.permission);
          })
        );
      } else {
        // TODO redirect to forbidden page
        return this.router.parseUrl("/error/forbidden");
        // console.log("returning true");
        // return true;
      }
    }
  }
  
}
