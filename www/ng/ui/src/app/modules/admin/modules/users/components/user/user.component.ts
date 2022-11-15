import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { ActivatedRoute, Router } from '@angular/router';
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
    const user_id = this.route.snapshot.paramMap.get("user_id") || uuidv4();
    this.formUser.get('user_id')?.setValue(user_id);
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

  generate_password() {
    console.log("UserComponent::generate_password()");
  }

  submit() {
    console.log("UserComponent::submit()");
    if (this.formUser.valid) {
      this.submitting = true;
      this.users_service.add(
        this.formUser.get('user_id')?.value || '',
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
