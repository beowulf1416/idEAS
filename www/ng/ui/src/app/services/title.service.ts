import { Injectable } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { environment } from 'src/environments/environment';

@Injectable({
  providedIn: 'root'
})
export class TitleService {

  app_name = '';

  constructor(
    private title: Title
  ) { 
    this.app_name = environment.app_name;
  }

  set_title(title: string) {
    this.title.setTitle(`${this.app_name} | ${title}`);
  }
}
