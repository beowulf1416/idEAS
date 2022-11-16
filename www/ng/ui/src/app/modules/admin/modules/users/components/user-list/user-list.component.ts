import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { TitleService } from 'src/app/services/title.service';
import { UsersService } from '../../services/users.service';


export interface User {
  id: string,
  active: boolean,
  email: string
};

@Component({
  selector: 'app-user-list',
  templateUrl: './user-list.component.html',
  styleUrls: ['./user-list.component.css']
})
export class UserListComponent implements OnInit {

  formFilter = new FormGroup({
    filter: new FormControl('', []),
    items: new FormControl(10, []),
    page: new FormControl(1, []) 
  });

  users = Array<User>();
  current_page = 1;

  constructor(
    private title: TitleService,
    private users_service: UsersService
  ) {
    this.title.set_title("Users");
  }

  ngOnInit(): void {
    console.log("UserListComponent::ngOnInit()");
    this.filter_users();
  }

  get filter() {
    return this.formFilter.get('filter');
  }

  get items() {
    return this.formFilter.get('items');
  }

  get page() {
    return this.formFilter.get('page');
  }

  filter_users() {
    this.users_service.fetch(
      this.formFilter.get('filter')?.value || '',
      this.formFilter.get('items')?.value || 10,
      this.current_page
    ).subscribe(r => {
      console.log("UserListComponent::filter_users()");
      if (r.success) {
        this.users = (r.data as { users: Array<User> }).users;
      } else {
        console.error('UserListComponent::filter_users()', r);
      }
    });
  }

  navigate_first() {
    this.current_page = 1;
    this.formFilter.get('page')?.setValue(this.current_page);
    this.filter_users();
  }

  navigate_last() {
    console.log("// TODO UserListComponent::navigate_last()");
    this.filter_users();
  }

  navigate_previous() {
    --this.current_page;
    if (this.current_page < 1) {
      this.current_page = 1;
    }
    this.formFilter.get('page')?.setValue(this.current_page);
    this.filter_users();
  }

  navigate_next() {
    this.current_page++;
    this.formFilter.get('page')?.setValue(this.current_page);
    this.filter_users();
  }

  submit() {
    console.debug("UserListComponent::submit()");
    this.filter_users();
  }
}
