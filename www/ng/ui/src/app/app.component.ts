import { Component } from '@angular/core';
import { Observable } from 'rxjs';
import { User } from './classes/user';
import { UserService } from './services/user.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {

  user$: Observable<User>

  constructor(
    private user_service: UserService
  ) {
    this.user$ = user_service.user$;
  }

  
}
