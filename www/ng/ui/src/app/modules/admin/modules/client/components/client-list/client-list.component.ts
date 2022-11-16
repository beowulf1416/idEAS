import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { TitleService } from 'src/app/services/title.service';
import { ClientService } from '../../services/client.service';


export interface Client {
  id: string,
  active: boolean,
  name: string,
  description: string,
  address: string,
  country_id: number,
  url: string
};

@Component({
  selector: 'app-client-list',
  templateUrl: './client-list.component.html',
  styleUrls: ['./client-list.component.css']
})
export class ClientListComponent implements OnInit {

  current_page = 1;

  formFilter = new FormGroup({
    filter: new FormControl('', []),
    items: new FormControl(10, []),
    page: new FormControl(1, []) 
  });

  clients: Array<Client> = [];

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
    let page = this.current_page;

    this.client_service.clients(
      this.filter_name?.value || '',
      false,
      this.formFilter.get('items')?.value || 10,
      this.current_page
    ).subscribe(r => {
      console.debug("ClientListComponent::get_clients()", r);
      if (r.success) {
        this.clients = r.data.clients;
      }
    });
  }

  navigate_first() {
    this.current_page = 1;
    this.formFilter.get('page')?.setValue(this.current_page);
    this.get_clients();
  }

  navigate_last() {
    console.log("// TODO UserListComponent::navigate_last()");
    this.get_clients();
  }

  navigate_previous() {
    --this.current_page;
    if (this.current_page < 1) {
      this.current_page = 1;
    }
    this.formFilter.get('page')?.setValue(this.current_page);
    this.get_clients();
  }

  navigate_next() {
    this.current_page++;
    this.formFilter.get('page')?.setValue(this.current_page);
    this.get_clients();
  }


  submit() {
    console.log("//TODO ClientListComponent::submit()");
  }
}
