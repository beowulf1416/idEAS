import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class AccountingService {

  constructor(
    private http: HttpClient
  ) { }
}
