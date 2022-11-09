import { Component, OnInit } from '@angular/core';
import { FormArray, FormBuilder, FormControl, FormGroup } from '@angular/forms';
import { ActivatedRoute } from '@angular/router';
import { MessageType } from 'src/app/classes/message-type';
import { MessageService } from 'src/app/services/message.service';
import { TitleService } from 'src/app/services/title.service';
import { ClientService } from '../../services/client.service';


@Component({
  selector: 'app-member-list',
  templateUrl: './member-list.component.html',
  styleUrls: ['./member-list.component.css']
})
export class MemberListComponent implements OnInit {

  client_id = '';
  members_active = Array<{
    id: string,
    active: boolean,
    email: string
  }>();
  members_inactive = Array<{
    id: string,
    active: boolean,
    email: string
  }>();


  formMembers = new FormGroup({
    active: new FormGroup({
      members: new FormArray([])
    })
  });

  constructor(
    private title: TitleService,
    private client_service: ClientService,
    private msg_service: MessageService,
    private route: ActivatedRoute,
    private fb: FormBuilder
  ) {}

  ngOnInit(): void {
    this.title.set_title("Members");

    this.client_id = this.route.snapshot.paramMap.get("client_id") || '';
    this.client_service.members(
      this.client_id,
      true
    ).subscribe(r => {
      console.log("MemberListComponent::ngOnInit()", r);
      if (r.success) {
        this.members_active = (r.data as { members: Array<{ id: string, active: boolean, email: string}> }).members;

        const members = (this.formMembers.get('active.members') as FormArray);
        this.members_active.forEach(m => {
          members.push(new FormControl(m.id, []));
        });
      } else {
        this.msg_service.send(r.message, MessageType.error);
      }
    });

    this.client_service.members(
      this.client_id,
      false
    ).subscribe(r => {
      console.log("MemberListComponent::ngOnInit()", r);
      if (r.success) {
        this.members_inactive = (r.data as { members: Array<{ id: string, active: boolean, email: string}> }).members;
      } else {
        this.msg_service.send(r.message, MessageType.error);
      }
    });
  }

  submit() {
    console.log("MemberListComponent::submit()");

    const members = this.formMembers.get('active.members');
    console.debug("members", members);
    console.log(this.formMembers.get('active')?.value);
  }
}