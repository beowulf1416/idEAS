import { Component, OnInit } from '@angular/core';
import { AbstractControl, FormControl, FormGroup, ValidationErrors, ValidatorFn, Validators } from '@angular/forms';


export const controlsIdenticalValidator: ValidatorFn = (control: AbstractControl): 
  ValidationErrors | null => {
    const email1 = control.get('email')?.value;
    const email2 = control.get('confirm')?.value;

    return email1 == email2 ? null : { controlsIdentical: true };
  }


@Component({
  selector: 'app-signup',
  templateUrl: './signup.component.html',
  styleUrls: ['./signup.component.css']
})
export class SignupComponent implements OnInit {

  signupForm = new FormGroup({
    email: new FormControl('', [
      Validators.required,
      Validators.email
    ]),
    confirm: new FormControl('', [
      Validators.required,
      Validators.required
    ])
  }, {
    validators: controlsIdenticalValidator
  });

  constructor() { }

  ngOnInit(): void {
  }

  get email() {
    return this.signupForm.get('email');
  }

  get confirm() {
    return this.signupForm.get('confirm');
  }

  signup() {
    if (this.signupForm.valid) {
      console.log("signup submit");
    } else {
      console.log("signup form is invalid");
    }
  }

}
