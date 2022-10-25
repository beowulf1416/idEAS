import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { ActivatedRoute } from '@angular/router';
import { User } from 'src/app/classes/user';
import { TitleService } from 'src/app/services/title.service';
import { UserService } from 'src/app/services/user.service';
import { RoleService } from '../../services/role.service';

import { v4 as uuidv4 } from 'uuid';
import { ApiResponse } from 'src/app/classes/api-response';


@Component({
  selector: 'app-role',
  templateUrl: './role.component.html',
  styleUrls: ['./role.component.css']
})
export class RoleComponent implements OnInit {

  submitting = false;
  action = "new";

  roleForm = new FormGroup({
    role_id: new FormControl('', []),
    client_id: new FormControl('', []),
    name: new FormControl('', [
      Validators.required
    ]),
    description: new FormControl('', [])
  });

  constructor(
    private title: TitleService,
    private user_service: UserService,
    private role_service: RoleService,
    private route: ActivatedRoute
  ) {
    this.title.set_title("Role");
  }

  get role_id() {
    return this.roleForm.get("role_id");
  }

  get client_id() {
    return this.roleForm.get("client_id");
  }

  get name() {
    return this.roleForm.get("name");
  }

  get description() {
    return this.roleForm.get("description");
  }

  ngOnInit(): void {
    this.roleForm.get("role_id")?.setValue(this.route.snapshot.paramMap.get("role_id") || uuidv4());
    this.action = this.route.snapshot.paramMap.get("action") || "new";

    this.user_service.get_user$().subscribe((user: User) => {
      this.roleForm.get("client_id")?.setValue(user.client);
    });
  }

  submit() {
    if (this.roleForm.valid) {
      this.submitting = true;
      switch (this.action) {
        case "edit": {
          this.role_service.update(
            this.roleForm.get("role_id")?.value || '',
            this.roleForm.get("client_id")?.value || '',
            this.roleForm.get("name")?.value || '',
            this.roleForm.get("description")?.value || ''
          ).subscribe((r: ApiResponse) => {
            console.log("RoleComponent::submit()", r);
            this.submitting = false;
          });
          break;
        } 
        default: {
          this.role_service.add(
            this.roleForm.get("role_id")?.value || '',
            this.roleForm.get("client_id")?.value || '',
            this.roleForm.get("name")?.value || '',
            this.roleForm.get("description")?.value || ''
          ).subscribe((r: ApiResponse) => {
            console.log("RoleComponent::submit()", r);
            this.submitting = false;
          });
          break;
        }
      }
      
    }
  }
}
