import { Component, OnInit } from '@angular/core';
import { AbstractControl, FormControl, FormGroup, ValidationErrors, ValidatorFn, Validators } from '@angular/forms';
import { Router } from '@angular/router';

import { TitleService } from 'src/app/services/title.service';
import { environment } from 'src/environments/environment';

import { SigninService } from '../../services/signin.service';


export function patternValidator(pattern: RegExp, error: ValidationErrors): ValidatorFn {
  return (control: AbstractControl): ValidationErrors | null => {
    if (!control.value) {
      return null;
    }

    const valid = pattern.test(control.value);
    return valid ? null : error;
  }
}


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
      patternValidator(new RegExp("[A-Z]"), { upper: true }),
      patternValidator(new RegExp("[0-9]"), { numeric: true })
    ])
  });

  formErrorText = '';
  formMsg = "";

  constructor(
    private title: TitleService,
    private service: SigninService,
    private router: Router
  ) { 
    this.title.set_title("Sign In");
  }

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
        if (r.body?.status == "success") {
          if (this.service.isSignedIn()) {
            this.service.getSignedInUser().subscribe(rs => {
              console.log("user service current user",rs);
              if (rs.status == "success") {
                if (rs.data?.tenants?.length > 1) {
                  console.log("//TODO redirect to tenant selection page");
                  this.router.navigate([environment.path_tenant_select]);
                } else {
                  console.log("//TODO redirect to dashboard");
                }
              } else {
                console.error(rs.message);
              }
            });
          } else {
            console.error(r.body?.message);
          }

          // this.formMsg = "Successfully signed in. Redirecting to home page in 3 seconds...";
          
          // let counter = 3;
          // let timer_id = setInterval(() => {
          //   this.formMsg = `Successfully signed in. Redirecting to home page in ${counter} seconds...`;
          //   counter--;
          // }, 1000);
          // setTimeout(() => {
          //   clearInterval(timer_id);
          //   this.router.navigate([environment.api_dashboard]);
          // }, 5000);  //5s
        } else {
          this.formErrorText = "failed authentication";
        }
      }, (error: any) => {
        console.error(error.message);
        this.formErrorText = error.message;
      });
    }
  }
}
