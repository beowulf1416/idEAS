import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { ApiResponse } from 'src/app/classes/api-response';
import { environment } from 'src/environments/environment';

@Injectable({
  providedIn: 'root'
})
export class ClientService {

  constructor(
    private http: HttpClient
  ) { }

  clients(): Observable<ApiResponse> {
    console.log("ClientService::clients()");
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_clients_fetch,
      {}
    );
  }
}
