import { Component } from '@angular/core';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-employee',
  templateUrl: './employee.component.html',
  styleUrls: ['./employee.component.css']
})
export class EmployeeComponent {


  constructor(
    private title: TitleService
  ) {
    this.title.set_title("Employee");
  }
}
