import { Component } from '@angular/core';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-account-chart',
  templateUrl: './account-chart.component.html',
  styleUrls: ['./account-chart.component.css']
})
export class AccountChartComponent {

  constructor(
    private title: TitleService
  ) {
    this.title.set_title("Chart of Accounts");
  }
}
