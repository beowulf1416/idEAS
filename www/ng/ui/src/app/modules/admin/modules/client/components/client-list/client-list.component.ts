import { Component, OnInit } from '@angular/core';
import { TitleService } from 'src/app/services/title.service';
import { ClientService } from '../../services/client.service';

@Component({
  selector: 'app-client-list',
  templateUrl: './client-list.component.html',
  styleUrls: ['./client-list.component.css']
})
export class ClientListComponent implements OnInit {

  clients: Array<{
    id: string,
    name: string
  }> = [];

  constructor(
    private title: TitleService,
    private client_service: ClientService
  ) {
    this.title.set_title("Clients");
  }

  ngOnInit(): void {
  }

  get_clients() {
    this.client_service.clients().subscribe(r => {
      console.debug("ClientListComponent::get_clients()", r);
    });
  }
}
