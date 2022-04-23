import { HttpClient } from '@angular/common/http';
import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { SigninService } from '../../services/signin.service';


export interface ApiResponse {
  status: string;
  message: string;
  data: any;
};

@Component({
  selector: 'app-signin',
  templateUrl: './signin.component.html',
  styleUrls: ['./signin.component.css']
})
export class SigninComponent implements OnInit {

  signinForm = new FormGroup({
    email: new FormControl('', [
      Validators.required,
      Validators.email
    ]),
    pw: new FormControl('', [
      Validators.minLength(8),
      Validators.pattern("[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}")
    ])
  });

  formErrorText = '';

  constructor(
    private service: SigninService
  ) { }

  ngOnInit(): void {
  }

  get email() {
    return this.signinForm.get("email");
  }

  get pw() {
    return this.signinForm.get("pw");
  }

  signin() {
    console.log("SigninComponent::signin()");
    console.log(this.signinForm);

    if (this.signinForm.valid) {

      console.log(this.signinForm);

      this.service.signIn(
        this.signinForm.get("email")?.value,
        this.signinForm.get("pw")?.value
      ).subscribe(r => {
        console.log(r);
      }, (error: any) => {
        console.error(error.message);
        this.formErrorText = error.message;
        this.signinForm.setErrors({ 'backend': true});
      });
    }
  }
}
