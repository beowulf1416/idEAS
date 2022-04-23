import { Component, OnInit } from '@angular/core';
import { AbstractControl, FormControl, FormGroup, ValidationErrors, Validator, ValidatorFn, Validators } from '@angular/forms';
import { SigninService } from '../../../signin/services/signin.service';


export function matchValidator(control1: string, control2: string): ValidatorFn {
  return (control: AbstractControl): ValidationErrors | null => {
    const value1 = control.get(control1)?.value;
    const value2 = control.get(control2)?.value;

    return value1 == value2 ? null : { identical: true };
  }
}

export function patternValidator(pattern: RegExp, error: ValidationErrors): ValidatorFn {
  return (control: AbstractControl): ValidationErrors | null => {
    if (!control.value) {
      return null;
    }

    const valid = pattern.test(control.value);
    return valid ? null : error;
  }
}

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
        // Validators.pattern("\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}\b")
        patternValidator(new RegExp("[A-Z]{2,}"), { upper: true }),
        patternValidator(new RegExp("[a-z]{2,}"), { lower: true }),
        patternValidator(new RegExp("[0-9]{2,}"), { number: true })
      ]),
      pw2: new FormControl('', [
        Validators.required,
        Validators.minLength(8),
        Validators.pattern("\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}\b")
      ])
    }, {
      validators: matchValidator("pw1", "pw2")
    })
  });

  constructor(
    private service: SigninService
  ) { }

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
