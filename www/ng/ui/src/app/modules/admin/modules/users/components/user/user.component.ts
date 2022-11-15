import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { ActivatedRoute, Router } from '@angular/router';
import { TitleService } from 'src/app/services/title.service';
import { UsersService } from '../../services/users.service';
import { v4 as uuidv4 } from 'uuid';


export interface User {
  id: string,
  active: boolean,
  email: string
};

@Component({
  selector: 'app-user',
  templateUrl: './user.component.html',
  styleUrls: ['./user.component.css']
})
export class UserComponent implements OnInit {

  submitting = false;
  formUser = new FormGroup({
    user_id: new FormControl('', []),
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
    private route: ActivatedRoute,
    private router: Router
  ) {
    this.title.set_title("User");
  }

  ngOnInit(): void {
    const user_id = this.route.snapshot.paramMap.get("user_id");
    if (user_id != null) {
      this.users_service.get(user_id).subscribe(r => {
        if (r.success) {
          this.set_user((r.data as { user: User }).user);
        }
      });
    }
  }

  get user_id() {
    return this.formUser.get('user_id');
  }

  get email() {
    return this.formUser.get('email');
  }

  get password() {
    return this.formUser.get('pw');
  }

  set_user(user: User) {
    this.formUser.get('user_id')?.setValue(user.id);
    this.formUser.get('email')?.setValue(user.email);
  }

  generate_password() {
    console.log("UserComponent::generate_password()");
  }

  submit() {
    console.log("UserComponent::submit()");
    if (this.formUser.valid) {
      this.submitting = true;

      const user_id = this.formUser.get('user_id')?.value;
      if (user_id == null) {
        this.users_service.add(
          uuidv4(),
          this.formUser.get('email')?.value || '',
          this.formUser.get('pw')?.value || ''
        ).subscribe(r => {
          if (r.success) {
            this.router.navigate(['/admin/users']);
          } else {
            console.error('UserComponent::add()', r);
          }
        });
      } else {
        this.users_service.update(
          user_id,
          this.formUser.get('email')?.value || '',
          this.formUser.get('pw')?.value || ''
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
}
