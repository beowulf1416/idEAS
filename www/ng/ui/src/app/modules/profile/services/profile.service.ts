import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { ApiResponse } from 'src/app/classes/api-response';

@Injectable({
  providedIn: 'root'
})
export class ProfileService {

  constructor(
    private http: HttpClient
  ) { }

  update(
    given_name: string,
    middle_name: string,
    family_name: string,
    prefix: string,
    suffix: string
  ): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(
      
    );
  }
}
