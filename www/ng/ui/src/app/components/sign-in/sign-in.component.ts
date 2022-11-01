import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { Router } from '@angular/router';
import { Subscription } from 'rxjs';
import { MessageType } from 'src/app/classes/message-type';
import { MessageService } from 'src/app/services/message.service';
import { TitleService } from 'src/app/services/title.service';
import { UserService } from 'src/app/services/user.service';

@Component({
  selector: 'app-sign-in',
  templateUrl: './sign-in.component.html',
  styleUrls: ['./sign-in.component.css']
})
export class SignInComponent implements OnInit {

  submitting = false;

  signinForm = new FormGroup({
    email: new FormControl('', [
      Validators.required,
      Validators.email
    ]),
    pw: new FormControl('', [
      Validators.required,
      Validators.minLength(8)
    ])
  });

  messages = '';
  msg_subscription: Subscription;

  constructor(
    private title: TitleService,
    private user_service: UserService,
    private msg_service: MessageService,
    private router: Router
  ) {
    this.title.set_title("Sign In");


    this.msg_subscription = this.msg_service.message$.subscribe(r => {
        this.messages = r.message;
    });
  }

  ngOnInit(): void {
  }

  get email() {
    return this.signinForm.get("email");
  }

  get pw() {
    return this.signinForm.get("pw");
  }

  submit() {
    console.log("SignInComponent::submit()");
    this.submitting = true;
    if (this.signinForm.valid) {
      this.user_service.sign_in(
        this.email?.value || '',
        this.pw?.value || ''
      ).subscribe(r => {
        console.debug("result", r);
        if (r.success) {
          console.log("redirecting...");
          this.router.navigate([""]);
        } else {
          console.error("SignInComponent::submit()", r);
          this.msg_service.send(r.message, MessageType.error);
        }
      })
    }
  }
}
