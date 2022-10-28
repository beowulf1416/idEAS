import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { TitleService } from 'src/app/services/title.service';
import { UserService } from 'src/app/services/user.service';

@Component({
  selector: 'app-sign-out',
  templateUrl: './sign-out.component.html',
  styleUrls: ['./sign-out.component.css']
})
export class SignOutComponent implements OnInit {

  remaining_seconds = 5;

  constructor(
    private title: TitleService,
    private user_service: UserService,
    private router: Router
  ) { 
    this.title.set_title("Sign Out");
  }

  ngOnInit(): void {
    this.user_service.sign_out();
    
    const self = this;
    setTimeout(() => {
      self.router.navigate(["sign-in"]);
    }, 5000);

    setInterval(() => {
      --self.remaining_seconds;
    }, 1000);
  }

}
