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

  submitting = false;

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

  get passwords() {
    return this.signUpForm.get("passwords");
  }

  get pw1() {
    return this.signUpForm.get("passwords.pw1");
  }

  get pw2() {
    return this.signUpForm.get("passwords.pw2");
  }

  submit() {
    console.log("SignUpComponent::submit()");
    if (this.signUpForm.valid) {
      this.submitting = true;
      this.user_service.sign_up(
        this.email1?.value || '',
        this.pw1?.value || ''
      ).subscribe(r => {
        console.debug("SignupComponent::submit()", r);

        this.submitting = false;
      });
    }
    
  }
}
