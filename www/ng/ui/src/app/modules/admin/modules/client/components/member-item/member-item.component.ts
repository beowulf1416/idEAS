import { Component, Input, OnInit } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';


export interface Member {
  id: string,
  active: boolean,
  email: string
}


@Component({
  selector: 'app-member-item',
  templateUrl: './member-item.component.html',
  styleUrls: ['./member-item.component.css']
})
export class MemberItemComponent implements OnInit {

  @Input() member!: Member; 

  formMemberItem = new FormGroup({
    id: new FormControl('', []),
    active: new FormControl(true, []),
    email: new FormControl('', [])
  });

  constructor() { }

  ngOnInit(): void {
  }

  get member_id() {
    return this.formMemberItem.get('id');
  }

  get active() {
    return this.formMemberItem.get('active');
  }

  get email() {
    return this.formMemberItem.get('email');
  }
}
