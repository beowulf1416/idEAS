import { Injectable } from '@angular/core';
import { ActivatedRouteSnapshot, CanActivate, Router, RouterStateSnapshot, UrlTree } from '@angular/router';
import { Observable } from 'rxjs';

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
        const permissions = this.user.get_permissions();
        if (permissions.includes(route.data?.permission)){
          return true;
        } else {
          return this.router.parseUrl("/error/forbidden");  
        }
      } else {
        // TODO redirect to forbidden page
        // return this.router.parseUrl("/error/forbidden");
        console.log("returning true");
        return true;
      }
    }
  }
  
}
