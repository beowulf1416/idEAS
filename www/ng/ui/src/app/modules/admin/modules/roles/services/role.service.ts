import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { ApiResponse } from 'src/app/classes/api-response';
import { environment } from 'src/environments/environment';


@Injectable({
  providedIn: 'root'
})
export class RoleService {

  constructor(
    private http: HttpClient
  ) { }

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
