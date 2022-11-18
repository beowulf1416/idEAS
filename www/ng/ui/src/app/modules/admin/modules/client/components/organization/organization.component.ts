import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-organization',
  templateUrl: './organization.component.html',
  styleUrls: ['./organization.component.css']
})
export class OrganizationComponent implements OnInit {


  formOrg = new FormGroup({
    name: new FormControl('', [
      Validators.required
    ]),
    description: new FormControl('', [])
  });

  constructor(
    private title: TitleService
  ) {
    this.title.set_title("Organization");
  }

  ngOnInit(): void {
  }

  submit() {
    console.log("OrganizationComponent::submit()");
  }

  get name() {
    return this.formOrg.get('name');
  }

  get description() {
    return this.formOrg.get('description');
  }
}
