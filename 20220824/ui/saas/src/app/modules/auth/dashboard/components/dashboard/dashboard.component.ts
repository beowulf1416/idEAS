import { Component, OnInit } from '@angular/core';
import { TitleService } from 'src/app/services/title.service';
import { UserService } from 'src/app/services/user.service';

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrls: ['./dashboard.component.css']
})
export class DashboardComponent implements OnInit {

  permissions: Array<string> = [];

  constructor(
    private title: TitleService,
    private user_service: UserService
  ) { 
    this.title.set_title("Dashboard");
  }

  ngOnInit(): void {
    this.user_service.get_permissions().subscribe((permissions: Array<string>) => {
      this.permissions = permissions;
    });
  }

  
}
