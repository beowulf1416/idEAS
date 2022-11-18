import { Component } from '@angular/core';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrls: ['./dashboard.component.css']
})
export class DashboardComponent {


  constructor(
    private title: TitleService
  ) {
    this.title.set_title("Human Resources");
  }
}
