import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { from, Observable } from 'rxjs';
import { catchError, map } from 'rxjs/operators';

import { ApiResponse } from 'src/app/services/user.service';
import { environment } from 'src/environments/environment';
import { Tenant } from '../classes/tenant';


@Injectable({
  providedIn: 'root'
})
export class TenantsService {

  constructor(
    private http: HttpClient
  ) { }

  get_tenants(
    filter: string,
    items: number,
    page: number
  ): Observable<Array<Tenant>> {
    return this.http.post<ApiResponse>(
      environment.api_base_url + environment.api_admin_tenants,
      {
        filter: filter,
        items: items,
        page: page
      }
    ).pipe(
        map((r: ApiResponse) => {
          console.log("TenantsService::get_tenants()");
          if (r.status == "success") {
            return r.data?.tenants;
          } else {
            return [];
          }
        }),
        catchError(e => {
          console.error("TenantsService::get_tenants()",e);
          return [];
        })
    )
  }
}
