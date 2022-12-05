import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { ActivatedRoute } from '@angular/router';
import { User } from 'src/app/classes/user';
import { TitleService } from 'src/app/services/title.service';
import { UserService } from 'src/app/services/user.service';
import { RoleService } from '../../services/role.service';

import { v4 as uuidv4 } from 'uuid';
import { ApiResponse } from 'src/app/classes/api-response';
import { MessageService } from 'src/app/services/message.service';
import { MessageType } from 'src/app/classes/message-type';
import { ActionType } from 'src/app/classes/action-type';


export interface Role {
  id: string,
  active: boolean,
  name: string,
  description: string
};

@Component({
  selector: 'app-role',
  templateUrl: './role.component.html',
  styleUrls: ['./role.component.css']
})
export class RoleComponent implements OnInit {

  submitting = false;
  action: ActionType = ActionType.new;

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
    private msg_service: MessageService,
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
    const role_id = this.route.snapshot.paramMap.get("role_id") || uuidv4();
    // this.roleForm.get("role_id")?.setValue(role_id);
    // this.action = this.route.snapshot.paramMap.get("action") || "new";
    this.action = (this.route.snapshot.data as { action: ActionType }).action;

    switch (this.action) {
      case ActionType.view: {
        this.fetch_role(role_id, true);
        break;
      }
      case ActionType.edit: {
        this.fetch_role(role_id, false);
        break;
      }
      default: {
        break;
      }
    }

    this.user_service.get_user$().subscribe((user: User) => {
      this.roleForm.get("client_id")?.setValue(user.client);
    });
  }

  fetch_role(
    role_id: string,
    read_only: boolean
  ) {
    this.role_service.get(role_id).subscribe(r => {
      if (r.success) {
        const role = (r.data as { role: Role}).role;
        this.set_role(role, read_only);
      } else {
        this.msg_service.send(r.message, MessageType.error);
      }
    });
  }

  set_role(
    role: Role,
    read_only: boolean
  ) {
    this.roleForm.get("role_id")?.setValue(role.id);
    this.roleForm.get("name")?.setValue(role.name);
    this.roleForm.get("description")?.setValue(role.description);
  }

  submit() {
    if (this.roleForm.valid) {
      this.submitting = true;
      switch (this.action) {
        case ActionType.edit: {
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
