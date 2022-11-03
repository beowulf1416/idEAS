import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { MessageType } from 'src/app/classes/message-type';
import { matchValidator } from 'src/app/classes/validators/match-validator';
import { patternValidator } from 'src/app/classes/validators/pattern-validator';
import { MessageService } from 'src/app/services/message.service';
import { TitleService } from 'src/app/services/title.service';
import { ProfileService } from '../../services/profile.service';

@Component({
  selector: 'app-password',
  templateUrl: './password.component.html',
  styleUrls: ['./password.component.css']
})
export class PasswordComponent implements OnInit {

  submitting = false;
  messages = "";

  formPassword = new FormGroup({
    pw1: new FormControl('', [
      Validators.required,
      Validators.minLength(8),
      patternValidator(new RegExp("[A-Z]*[A-Z]"), { upper: true }),
      patternValidator(new RegExp("[0-9]*[0-9]"), { numeric: true })
    ]),
    pw2: new FormControl('', [
      Validators.required
    ])
  },{
    validators: [
      matchValidator("pw1", "pw2")
    ]
  });

  constructor(
    private title: TitleService,
    private profile_service: ProfileService,
    private msg_service: MessageService
  ) {
    this.title.set_title("Password");
  }

  ngOnInit(): void {
  }

  get pw1() {
    return this.formPassword.get("pw1");
  }

  get pw2() {
    return this.formPassword.get("pw2");
  }

  submit() {
    console.log("PasswordComponent::submit()");
    if (this.formPassword.valid) {
      this.submitting = true;
      this.profile_service.set_password(
        this.pw1?.value || ''
      ).subscribe(r => {
        if (r.success) {
          this.messages = r.message;
        } else {
          this.messages = r.message;
          this.msg_service.send(r.message, MessageType.error);
        }
        this.submitting = false;
      });
    }
  }
}
