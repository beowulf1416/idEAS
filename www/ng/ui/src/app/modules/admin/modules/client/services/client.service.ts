import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { ApiResponse } from 'src/app/classes/api-response';
import { environment } from 'src/environments/environment';

import { v4 as uuidv4 } from 'uuid';

@Injectable({
  providedIn: 'root'
})
export class ClientService {

  constructor(
    private http: HttpClient
  ) { }

  clients(
    filter: string,
    active: boolean,
    items: number,
    page: number
  ): Observable<ApiResponse> {
    console.log("ClientService::clients()");
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_clients_fetch,
      {
        filter: filter,
        active: active,
        items: items,
        page: page
      }
    );
  }

  add(
    name: string,
    active: boolean,
    description: string,
    address: string,
    country_id: number,
    url: string
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_clients_add,
      {
        id: uuidv4(),
        name: name,
        active: active,
        description: description,
        address: address,
        country_id: country_id,
        url: url
      }
    );
  }

  get(
    client_id: string
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_clients_get,
      {
        id: client_id
      }
    );
  }

  update(
    id: string,
    name: string,
    active: boolean,
    description: string,
    address: string,
    country_id: number,
    url: string
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_clients_update,
      {
        id: id,
        name: name,
        active: active,
        description: description,
        address: address,
        country_id: country_id,
        url: url
      }
    );
  }

  members(
    client_id: string,
    active: boolean
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_client_members_fetch,
      {
        client_id: client_id,
        active: active
      }
    )
  }

  invite_member(
    client_id: string,
    email: string
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_client_members_invite,
      {
        client_id: client_id,
        email: email
      }
    );
  }
}
