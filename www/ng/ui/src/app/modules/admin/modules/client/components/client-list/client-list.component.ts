import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
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
    active: boolean,
    name: string,
    description: string,
    address: string,
    country_id: number,
    url: string
  }> = [];

  formClients = new FormGroup({
    name: new FormControl('', []),
    page: new FormControl('', [
      Validators.required
    ]),
    items: new FormControl('', [])
  });

  constructor(
    private title: TitleService,
    private client_service: ClientService
  ) {
    this.title.set_title("Clients");
  }

  ngOnInit(): void {
    this.get_clients();
  }

  get filter_name() {
    return this.formClients.get("name");
  }

  get filter_items() {
    return this.formClients.get("items")
  }

  get filter_page() {
    return this.formClients.get("page");
  }

  get_clients() {
    let items = parseInt(this.filter_items?.value || '');
    let page = parseInt(this.filter_page?.value || '');

    this.client_service.clients(
      this.filter_name?.value || '',
      false,
      isNaN(items) ? 10 : items,
      isNaN(page) ? 0 : page
    ).subscribe(r => {
      console.debug("ClientListComponent::get_clients()", r);
      if (r.success) {
        this.clients = r.data.clients;
      }
    });
  }
}
