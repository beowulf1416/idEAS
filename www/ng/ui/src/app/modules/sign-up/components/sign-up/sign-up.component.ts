import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { matchValidator } from 'src/app/classes/validators/match-validator';
import { patternValidator } from 'src/app/classes/validators/pattern-validator';
import { TitleService } from 'src/app/services/title.service';
import { SignUpService } from '../../services/sign-up.service';

@Component({
  selector: 'app-sign-up',
  templateUrl: './sign-up.component.html',
  styleUrls: ['./sign-up.component.css']
})
export class SignUpComponent implements OnInit {

  signUpForm = new FormGroup({
    emails: new FormGroup({
      email1: new FormControl('', [
        Validators.required,
        Validators.email
      ]),
      email2: new FormControl('', [
        Validators.required
      ])
    }, {
      validators: [
        matchValidator("email1", "email2")
      ]
    }),
    passwords: new FormGroup({
      pw1: new FormControl('', [
        Validators.required,
        Validators.minLength(8),
        patternValidator(new RegExp("[A-Z]{2,}"), { upper: true }),
        patternValidator(new RegExp("[0-9]{2,}"), { numeric: true })
      ]),
      pw2: new FormControl('', [
        Validators.required
      ])
    },{
      validators: [
        matchValidator("pw1", "pw2")
      ]
    })
  });

  constructor(
    private title: TitleService,
    private user_service: SignUpService
  ) {
    this.title.set_title("Sign Up");
  }

  ngOnInit(): void {
  }

  get emails() {
    return this.signUpForm.get("emails");
  }

  get email1() {
    return this.signUpForm.get("emails.email1");
  }

  get email2() {
    return this.signUpForm.get("emails.email1");
  }

  submit() {
    console.log("SignUpComponent::submit()");
    
    
  }
}
