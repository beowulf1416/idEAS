import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-start',
  templateUrl: './start.component.html',
  styleUrls: ['./start.component.css']
})
export class StartComponent implements OnInit {

  registerForm = new FormGroup({
    email: new FormControl('', [
      Validators.email,
      Validators.required
    ]),
    confirm_email: new FormControl('', [
      Validators.required
    ])
  });

  constructor(
    private title: TitleService
  ) { 
    this.title.set_title('Register');
  }

  ngOnInit(): void {
  }

  submit() {
    console.log('StartComponent::submit()');
  }
}
