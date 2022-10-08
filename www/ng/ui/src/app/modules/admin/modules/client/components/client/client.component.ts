import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-client',
  templateUrl: './client.component.html',
  styleUrls: ['./client.component.css']
})
export class ClientComponent implements OnInit {

  clientForm = new FormGroup({
    name: new FormControl('', []),
    description: new FormControl('', []),
    address: new FormControl('', []),
    country: new FormControl('', []),
    url: new FormControl('', [])
  });

  constructor(
    private title: TitleService
  ) {
    this.title.set_title("Client");
  }

  ngOnInit(): void {
  }

  submit() {
    console.log("ClientComponent::submit()");
  }
}
