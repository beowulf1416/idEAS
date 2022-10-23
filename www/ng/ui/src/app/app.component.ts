import { Component, OnInit } from '@angular/core';
import { BehaviorSubject, Observable, Subscription } from 'rxjs';
import { User } from './classes/user';
import { UserService } from './services/user.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent implements OnInit {

  // user$: Observable<User>;
  user_subscription: Subscription;
  user: User = new User('', '', '');

  constructor(
    private user_service: UserService
  ) {
    console.log("AppComponent::constructor()");
    // this.user$ = user_service.get_user$();
    // user_service.user$.subscribe(r => {
    //   this.user$ = r;
    // });
    this.user_subscription = this.user_service.get_user$().subscribe(r => {
      // console.debug("AppComponent::constructor() r", r);
      this.user = new User(r.email_address, r.fullname, r.get_client_name);
    });
  }

  ngOnInit(): void {
    // console.log("AppComponent::ngOnInit()");
    // this.user$ = this.user_service.user$;
  }
}
