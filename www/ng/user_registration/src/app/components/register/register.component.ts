import { Component, OnInit } from '@angular/core';
import { UntypedFormControl, UntypedFormGroup, Validators } from '@angular/forms';
import { TitleService } from 'src/app/services/title.service';

import { uuid } from 'uuidv4';

@Component({
  selector: 'app-register',
  templateUrl: './register.component.html',
  styleUrls: ['./register.component.css']
})
export class RegisterComponent implements OnInit {

  registerForm = new UntypedFormGroup({
    email: new UntypedFormControl('', [
      Validators.required,
      Validators.email
    ]),
    confirm_email: new UntypedFormControl('', [
      Validators.required
    ])
  });

  constructor(
    private title: TitleService
  ) { }

  ngOnInit(): void {
    this.title.set_title("Sign Up");
  }

  get email() {
    return this.registerForm.get('email');
  }

  get confirm_email() {
    return this.registerForm.get('confirm_email');
  }

  submit() {
    console.log('RegisterComponent::submit()');
  }
}
