import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';

import { matchValidator } from 'src/app/classes/validators/match-validator';
import { patternValidator } from 'src/app/classes/validators/pattern-validator';
import { TitleService } from 'src/app/services/title.service';
import { SigninService } from '../../../signin/services/signin.service';


@Component({
  selector: 'app-signup',
  templateUrl: './signup.component.html',
  styleUrls: ['./signup.component.css']
})
export class SignupComponent implements OnInit {

  signupForm = new FormGroup({
    emails: new FormGroup({
      email: new FormControl('', [
        Validators.required,
        Validators.email
      ]),
      confirm: new FormControl('', [
        Validators.required
      ])
    },{
      validators: matchValidator("email", "confirm")
    }),
    passwords: new FormGroup({
      pw1: new FormControl('', [
        Validators.required,
        Validators.minLength(8),
        patternValidator(new RegExp("[A-Z]{2,}"), { upper: true }),
        patternValidator(new RegExp("[0-9]{2,}"), { numeric: true })
      ]),
      pw2: new FormControl('', [
        Validators.required,
        Validators.minLength(8)
      ])
    }, {
      validators: matchValidator("pw1", "pw2")
    })
  });

  constructor(
    private title: TitleService,
    private service: SigninService
  ) {
    this.title.set_title("Sign Up");
  }

  ngOnInit(): void {
  }

  get email() {
    return this.signupForm.get('emails.email');
  }

  get confirm() {
    return this.signupForm.get('emails.confirm');
  }

  get emails() {
    return this.signupForm.get("emails");
  }

  get pw1() {
    return this.signupForm.get("passwords.pw1");
  }

  get pw2() {
    return this.signupForm.get("passwords.pw2");
  }

  get passwords() {
    return this.signupForm.get("passwords");
  }

  signup() {
    console.log('signup()');
    if (this.signupForm.valid) {
      this.service.signIn(
        this.signupForm.get("emails.email")?.value,
        this.signupForm.get("passwords.pw1")?.value
      ).subscribe(r => {
        console.log(r);
      });
    } else {
      console.log("signup form is invalid");
    }
  }

}
