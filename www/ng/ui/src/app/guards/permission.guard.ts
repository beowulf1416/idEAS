import { Injectable } from '@angular/core';
import { ActivatedRouteSnapshot, CanActivate, RouterStateSnapshot, UrlTree } from '@angular/router';
import { map, Observable, of } from 'rxjs';
import { MessageService } from '../services/message.service';
import { UserService } from '../services/user.service';

@Injectable({
  providedIn: 'root'
})
export class PermissionGuard implements CanActivate {

  constructor(
    private user_service: UserService,
    private msg_service: MessageService
  ) {}

  canActivate(
    route: ActivatedRouteSnapshot,
    state: RouterStateSnapshot): Observable<boolean | UrlTree> | Promise<boolean | UrlTree> | boolean | UrlTree {

    return this.user_service.get_user$().pipe(
      map(r => {
        const found = r.permissions.find(x => x == route.data['permission']);
        console.log("PermissionGuard::canActivate()", found);
        return found != undefined;
      })
    );
  }
  
}
