import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { MessageType } from 'src/app/classes/message-type';
import { MessageService } from 'src/app/services/message.service';
import { TitleService } from 'src/app/services/title.service';
import { RoleService } from '../../services/role.service';


export interface Role {
  id: string,
  active: boolean,
  name: string,
  description: string 
}


@Component({
  selector: 'app-role-list',
  templateUrl: './role-list.component.html',
  styleUrls: ['./role-list.component.css']
})
export class RoleListComponent implements OnInit {

  client_id = '';
  roles: Array<Role> = new Array<Role>();
  roles_filter: Array<Role> = new Array<Role>();

  constructor(
    private title: TitleService,
    private role_service: RoleService,
    private message_service: MessageService,
    private route: ActivatedRoute
  ) {
    this.title.set_title("Roles");
  }

  ngOnInit(): void {
    this.client_id = this.route.snapshot.paramMap.get("client_id") || '';
    this.role_service.fetch(
      this.client_id,
      '',
      true,
      100,
      0
    ).subscribe(r => {
      if (r.success) {
        this.roles = (r.data as { roles: Array<Role> }).roles;
      } else {
        this.message_service.send(r.message, MessageType.error);
      }
    });
  }
}
