import { Component, OnInit } from '@angular/core';
import { BehaviorSubject, Observable, Subscription } from 'rxjs';
import { User } from './classes/user';
import { MessageService } from './services/message.service';
import { UserService } from './services/user.service';
// import {
//   MatSnackBar,
//   MatSnackBarHorizontalPosition,
//   MatSnackBarVerticalPosition,
// } from '@angular/material/legacy-snack-bar';
import { MatSnackBar } from '@angular/material/snack-bar';

import { v4 as uuidv4 } from 'uuid';
import { MessageType } from './classes/message-type';
import { environment } from 'src/environments/environment.prod';


@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent implements OnInit {

  app_name = environment.app_name;

  // user$: Observable<User>;
  user_subscription: Subscription;
  user: User = new User(
    '', 
    '', 
    '', 
    [],
    []
  );

  msg_subscription: Subscription;
  messages = new Array<{
    id: string,
    message: string,
    type: MessageType
  }>();

  constructor(
    private sb: MatSnackBar,
    private user_service: UserService,
    private msg_service: MessageService
  ) {
    console.log("AppComponent::constructor()");
    this.user_subscription = this.user_service.get_user$().subscribe(r => {
      console.debug("result", r);
      this.user = new User(
        r.email, 
        r.name, 
        r.client,
        r.clients,
        []
      );
    });

    this.msg_subscription = this.msg_service.message$.subscribe(r => {
      if (r.message != '') {
        this.messages.push({
          id: uuidv4(),
          message: r.message,
          type: r.type
        });

        this.sb.open(
          r.message,
          '',
          {
            duration: 3000,
            horizontalPosition: 'end',
            verticalPosition: 'top'
          }
        );
      }
    });
  }

  ngOnInit(): void {
    // console.log("AppComponent::ngOnInit()");
    // this.user$ = this.user_service.user$;
  }

  /// TODO remake this
  close_message(id: string){
    console.log("AppComponent::close_message()", id);
    // const msg = this.messages.find(m => m.id == id);
    const ix = 0;
    this.messages.find((v, i) => {
      if (v.id == id) {
        delete this.messages[i];
      }
    });
  }

  switch_client(client_id: string){
    console.log("AppComponent::switch_client()", client_id);
    this.user_service.set_client(client_id);
  }
}
