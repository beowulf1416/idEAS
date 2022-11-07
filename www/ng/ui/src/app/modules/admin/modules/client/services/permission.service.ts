import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { ApiResponse } from 'src/app/classes/api-response';
import { environment } from 'src/environments/environment.prod';

@Injectable({
  providedIn: 'root'
})
export class PermissionService {

  constructor(
    private http: HttpClient
  ) { }

  assigned(
    role_id: string
  ) {
    this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_permissions_assigned,
      {
        role_id: role_id
      }
    ).subscribe(r => {
      console.debug("PermissionService::assigned()", r);
    });
  }

  not_assigned(
    role_id: String
  ) {
    this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_permissions_not_assigned,
      {
        role_id: role_id
      }
    ).subscribe(r => {
      console.debug("PermissionService::not_assigned()", r);
    });
  }
}
