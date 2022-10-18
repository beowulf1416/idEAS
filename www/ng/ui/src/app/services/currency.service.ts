import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { Currency } from '../classes/currency';

@Injectable({
  providedIn: 'root'
})
export class CurrencyService {

  constructor(
    private http: HttpClient
  ) { }

  fetch(): Observable<Array<Currency>> {
    let self = this;
    return new Observable<Array<Currency>>(function(observer) {
      
    });
  }
}
