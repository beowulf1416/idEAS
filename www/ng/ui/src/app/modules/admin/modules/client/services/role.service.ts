import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { ApiResponse } from 'src/app/classes/api-response';
import { environment } from 'src/environments/environment.prod';

@Injectable({
  providedIn: 'root'
})
export class RoleService {

  constructor(
    private http: HttpClient
  ) { }

  fetch(
    client_id: string,
    filter: string,
    active: boolean,
    items: number,
    page: number
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_roles_fetch,
      {
        client_id: client_id,
        filter: filter,
        active: active,
        items: items,
        page: page
      }
    );
  }

  add(
    id: string,
    client_id: string,
    name: string,
    description: string
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_role_add,
      {
        id: id,
        client_id: client_id,
        name: name,
        description: description
      }
    )
  }

  update(
    id: string,
    client_id: string,
    name: string,
    description: string
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_role_update,
      {
        id: id,
        client_id: client_id,
        name: name,
        description: description
      }
    )
  }

  get(
    role_id: string
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_role_get,
      {
        role_id: role_id
      }
    );
  }
}
