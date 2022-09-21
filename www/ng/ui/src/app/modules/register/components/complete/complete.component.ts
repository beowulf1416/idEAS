import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-complete',
  templateUrl: './complete.component.html',
  styleUrls: ['./complete.component.css']
})
export class CompleteComponent implements OnInit {

  completeForm = new FormGroup({
    email: new FormControl('', []),
    
    prefix: new FormControl('', []),
    suffix: new FormControl('', []),

    given_name: new FormControl('', []),
    middle_name: new FormControl('', []),
    family_name: new FormControl('', [])
  });

  constructor(
    private title: TitleService
  ) { 
    this.title.set_title('Complete Registration');
  }

  ngOnInit(): void {
  }

  submit() {
    console.log('CompleteComponent::submit()');
  }
}
