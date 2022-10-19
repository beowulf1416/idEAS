import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { catchError, filter, finalize, map, Observable } from 'rxjs';
import { environment } from 'src/environments/environment';
import { ApiResponse } from '../classes/api-response';
import { Country } from '../classes/country';

@Injectable({
  providedIn: 'root'
})
export class CountryService {

  constructor(
    private http: HttpClient
  ) { }

  fetch(): Observable<Array<Country>> {
    console.log("CountryService::fetch()");
    let self = this;
    // return new Observable<Array<Country>>(function(observer) {
    //   self.http.post<ApiResponse>(
    //     environment.api_url_base + environment.api_country_fetch,
    //     {}
    //   ).pipe(
    //     filter(r => r.success),
    //     map(r => {
    //       console.debug("CountryService::fetch()", r);
    //       let countries = (r.data as { countries: Array<Country> }).countries;
    //       observer.next(countries);
    //     }),
    //     catchError(e => {
    //       console.error("CountryService::fetch()", e);
    //       // observer.next(new Array<Country>());
    //       return new Array<Country>();
    //     }),
    //     finalize(() => {
    //       observer.complete();
    //     })
    //   );
    // });

    return this.http.post<ApiResponse>(
      environment.api_url_base + environment.api_country_fetch,
      {}
    ).pipe(
      filter(r => r.success),
      map(r => {
        console.debug(r);
        let countries = (r.data as { countries: Array<Country> }).countries;
        return countries; 
      })
    );
  }
}
