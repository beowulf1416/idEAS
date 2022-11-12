import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { Router } from '@angular/router';
import { TitleService } from 'src/app/services/title.service';
import { UsersService } from '../../services/users.service';
import { v4 as uuidv4 } from 'uuid';

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
    private users_service: UsersService,
    private router: Router
  ) {
    this.title.set_title("User");
  }

  ngOnInit(): void {
  }

  get email() {
    return this.formUser.get('email');
  }

  get password() {
    return this.formUser.get('password');
  }

  generate_password() {
    console.log("UserComponent::generate_password()");
  }

  submit() {
    console.log("UserComponent::submit()");
    if (this.formUser.valid) {
      this.submitting = true;
      this.users_service.add(
        uuidv4(),
        this.formUser.get('email')?.value || '',
        this.formUser.get('password')?.value || ''
      ).subscribe(r => {
        if (r.success) {
          this.router.navigate(['/admin/users']);
        } else {
          console.error('UserComponent::add()', r);
        }
      });
    }
  }
}
