import { Component } from '@angular/core';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-employee-list',
  templateUrl: './employee-list.component.html',
  styleUrls: ['./employee-list.component.css']
})
export class EmployeeListComponent {


  constructor(
    private title: TitleService
  ) {
    this.title.set_title("Employees");
  }
}
