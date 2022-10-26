import { Component, OnInit } from '@angular/core';
import { ClientService } from 'src/app/services/client.service';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-client',
  templateUrl: './client.component.html',
  styleUrls: ['./client.component.css']
})
export class ClientComponent implements OnInit {

  clients: Array<{
    id: string,
    name: string
  }> = new Array();

  constructor(
    private title: TitleService,
    private client_service: ClientService
  ) { }

  ngOnInit(): void {
    this.title.set_title("Join Client");
    this.fetch_clients();
  }

  fetch_clients() {
    this.client_service.clients(
      "%",
      true,
      10,
      1
    ).subscribe(r => {
      console.log("ClientComponent::refresh_clients()", r);
    });
  }
}
