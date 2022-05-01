import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
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

    ]),
    pw2: new FormControl('', [

    ])
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

  submit() {
    console.log("PasswordComponent::submit()");
    this.password_service.change(
      this.passwordForm.get("pw1")?.value
    ).subscribe(r => {
      console.log(r);
    });
  }
}
