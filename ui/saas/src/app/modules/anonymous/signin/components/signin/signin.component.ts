import { HttpClient } from '@angular/common/http';
import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { environment } from 'src/environments/environment';
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
      Validators.pattern("\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}\b")
    ])
  });

  constructor(
    private service: SigninService
  ) { }

  ngOnInit(): void {
  }

  signin() {
    console.log("SigninComponent::signin()");
    this.service.signIn(
      this.signinForm.get("email")?.value,
      this.signinForm.get("pw")?.value
    ).subscribe(r => {
      console.log(r);
    });
  }
}
