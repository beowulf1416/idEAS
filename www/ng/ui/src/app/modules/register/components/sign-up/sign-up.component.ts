import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { matchValidator } from 'src/app/classes/validators/match-validator';
import { patternValidator } from 'src/app/classes/validators/pattern-validator';
import { TitleService } from 'src/app/services/title.service';

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
    }, {
      validators: [
        matchValidator("pw1", "pw2")
      ]
    })
  });

  constructor(
    private title: TitleService
  ) {
    this.title.set_title("Sign Up");
  }

  ngOnInit(): void {
  }

  submit() {
    console.log("SignUpComponent::submit()");
  }
}
