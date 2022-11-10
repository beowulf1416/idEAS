import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { TitleService } from 'src/app/services/title.service';
import { UsersService } from '../../services/users.service';

@Component({
  selector: 'app-user',
  templateUrl: './user.component.html',
  styleUrls: ['./user.component.css']
})
export class UserComponent implements OnInit {

  submitting = false;
  formUser = new FormGroup({
    email: new FormControl('', [
      Validators.required,
      Validators.email
    ]),
    pw: new FormControl('', [
      Validators.required,
      Validators.minLength(8)
    ])
  });

  constructor(
    private title: TitleService,
    private users_service: UsersService
  ) {
    this.title.set_title("User");
  }

  ngOnInit(): void {
  }

  generate_password() {
    console.log("UserComponent::generate_password()");
  }

  submit() {
    console.log("UserComponent::submit()");
  }
}
