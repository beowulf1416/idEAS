import { Component } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-account',
  templateUrl: './account.component.html',
  styleUrls: ['./account.component.css']
})
export class AccountComponent {


  formAcct = new FormGroup({
    name: new FormControl('', [
      Validators.required
    ]),
    description: new FormControl('', [])
  });

  constructor(
    private title: TitleService
  ) {
    this.title.set_title("Account");
  }

  submit() {
    console.log("AccountComponent::submit()");
  }

  get name() {
    return this.formAcct.get('name');
  }

  get description() {
    return this.formAcct.get('description');
  }
}
