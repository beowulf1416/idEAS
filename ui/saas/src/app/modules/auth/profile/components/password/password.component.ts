import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { matchValidator } from 'src/app/classes/validators/match-validator';
import { patternValidator } from 'src/app/classes/validators/pattern-validator';

import { TitleService } from 'src/app/services/title.service';
import { PasswordService } from '../../services/password.service';

@Component({
  selector: 'app-password',
  templateUrl: './password.component.html',
  styleUrls: ['./password.component.css']
})
export class PasswordComponent implements OnInit {

  passwordForm = new FormGroup({
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
    validators: matchValidator("pw1", "pw2")
  });

  constructor(
    private title: TitleService,
    private password_service: PasswordService
  ) { 
    this.title.set_title('Passwords');
  }

  ngOnInit(): void {
  }

  get pw1() {
    return this.passwordForm.get("pw1");
  }

  get pw2() {
    return this.passwordForm.get("pw2");
  }

  submit() {
    console.log("PasswordComponent::submit()");
    if (this.passwordForm.valid) {
      this.password_service.change(
        this.passwordForm.get("pw1")?.value
      ).subscribe(r => {
        console.log(r);
      });
    }
  }
}
