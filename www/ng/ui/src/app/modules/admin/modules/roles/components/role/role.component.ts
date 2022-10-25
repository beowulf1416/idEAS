import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-role',
  templateUrl: './role.component.html',
  styleUrls: ['./role.component.css']
})
export class RoleComponent implements OnInit {

  submitting = false;

  roleForm = new FormGroup({
    name: new FormControl('', [
      Validators.required
    ]),
    description: new FormControl('', [])
  });

  constructor(
    private title: TitleService
  ) {
    this.title.set_title("Role");
  }

  get name() {
    return this.roleForm.get("name");
  }

  get description() {
    return this.roleForm.get("description");
  }

  ngOnInit(): void {
  }

  submit() {

  }
}
