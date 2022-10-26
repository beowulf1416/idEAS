import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { ClientService } from 'src/app/services/client.service';

@Component({
  selector: 'app-client-select',
  templateUrl: './client-select.component.html',
  styleUrls: ['./client-select.component.css']
})
export class ClientSelectComponent implements OnInit {

  clients = [
    {
      id: 1,
      name: "client 1"
    },
    {
      id: 2,
      name: "client 2"
    },
    {
      id: 3,
      name: "client 3"
    },
    {
      id: 4,
      name: "client 4"
    },
    {
      id: 5,
      name: "client 5"
    },
    {
      id: 6,
      name: "client 6"
    }
  ];

  constructor(
    private client_service: ClientService
  ) { }

  ngOnInit(): void {
  }

  fetch_clients() {
    this.client_service.clients(
      "%",
      true,
      10,
      1
    ).subscribe(r => {
      console.log("ClientSelectComponent::fetch_clients()", r);
    });
  }
}
