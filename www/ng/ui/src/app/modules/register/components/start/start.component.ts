import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { TitleService } from 'src/app/services/title.service';
import { RegistrationService } from '../../services/registration.service';

import { v4 as uuidv4 } from 'uuid';

@Component({
  selector: 'app-start',
  templateUrl: './start.component.html',
  styleUrls: ['./start.component.css']
})
export class StartComponent implements OnInit {

  flag_submitting = false;

  registerForm = new FormGroup({
    email: new FormControl('', [
      Validators.email,
      Validators.required
    ]),
    confirm_email: new FormControl('', [
      Validators.required
    ])
  });

  constructor(
    private title: TitleService,
    private register_service: RegistrationService
  ) { 
    this.title.set_title('Register');
  }

  ngOnInit(): void {
  }

  get email() {
    return this.registerForm.get('email');
  }

  submit() {
    console.log('StartComponent::submit()');
    if (this.registerForm.valid) {
      this.flag_submitting = true;

      this.register_service.register(
        uuidv4(),
        this.registerForm.get('email')?.value || ''
      ).subscribe(r => {
        console.log(r);

        this.flag_submitting = false;
      });
    }
  }
}
