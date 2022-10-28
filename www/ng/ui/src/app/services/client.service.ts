import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { catchError, map, Observable, of } from 'rxjs';
import { environment } from 'src/environments/environment.prod';
import { ApiResponse } from '../classes/api-response';
import { MessageType } from '../classes/message-type';
import { MessageService } from './message.service';

@Injectable({
  providedIn: 'root'
})
export class ClientService {

  constructor(
    private http: HttpClient,
    private msg_service: MessageService
  ) { }

  clients(
    filter: string,
    active_only: boolean,
    items: number,
    page: number
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_clients_fetch,
      {
        filter: filter,
        active: active_only,
        items: items,                                               
        page: page
      }
    ).pipe(
      map(r => {
        console.log("ClientService::clients()", r);
        return r;
      }),
      catchError((e) => {
        console.error("ClientService::clients()", e);
        this.msg_service.send(e.statusText, MessageType.error);
        return of({
          success: false, 
          message: "unknown error", 
          data: null
        });
      })
    );
  }

  user_join(
    client_id: string
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_user_client_add,
      {
        client_id: client_id
      }
    );
  }
}
