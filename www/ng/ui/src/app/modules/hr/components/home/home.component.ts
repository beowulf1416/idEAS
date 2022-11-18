import { Component } from '@angular/core';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.css']
})
export class HomeComponent {

  constructor(
    private title: TitleService
  ) {
    this.title.set_title("Human Resources");
  }
}
