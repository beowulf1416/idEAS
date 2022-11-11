import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { TitleService } from 'src/app/services/title.service';
import { UsersService } from '../../services/users.service';

@Component({
  selector: 'app-user-list',
  templateUrl: './user-list.component.html',
  styleUrls: ['./user-list.component.css']
})
export class UserListComponent implements OnInit {

  formFilter = new FormGroup({
    filter: new FormControl('', [])
  });

  constructor(
    private title: TitleService,
    private users_service: UsersService
  ) {
    this.title.set_title("Users");
  }

  ngOnInit(): void {
  }

}
