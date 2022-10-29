import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-general',
  templateUrl: './general.component.html',
  styleUrls: ['./general.component.css']
})
export class GeneralComponent implements OnInit {

  profileForm = new FormGroup({
    first_name: new FormControl('', [
      Validators.required
    ]),
    middle_name: new FormControl('', []),
    last_name: new FormControl('', [
      Validators.required
    ]),
    prefix: new FormControl('', []),
    suffix: new FormControl('', [])
  });

  constructor(
    private title: TitleService
  ) {
    this.title.set_title("Profile");
  }

  ngOnInit(): void {
  }

  submit() {
    console.log("GeneralComponent::submit()");
  }
}
