import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { ActivatedRoute } from '@angular/router';
import { ApiResponse } from 'src/app/classes/api-response';
import { Client } from 'src/app/classes/client';
import { Country } from 'src/app/classes/country';
import { CountryService } from 'src/app/services/country.service';
import { TitleService } from 'src/app/services/title.service';
import { ClientService } from '../../services/client.service';

@Component({
  selector: 'app-client',
  templateUrl: './client.component.html',
  styleUrls: ['./client.component.css']
})
export class ClientComponent implements OnInit {

  submitting = false;

  client_id = "";
  action = "new";

  countries: Array<Country> = [];
  client: Client = {
    id: '',
    name: '',
    description: '',
    active: false,
    address: '',
    country_id: 0,
    url: ''
  };

  clientForm = new FormGroup({
    name: new FormControl('', []),
    active: new FormControl(false, []),
    description: new FormControl('', []),
    address: new FormControl('', []),
    country: new FormControl(0, []),
    url: new FormControl('', [])
  });

  constructor(
    private title: TitleService,
    private client_service: ClientService,
    private country_service: CountryService,
    private route: ActivatedRoute
  ) {
    this.title.set_title("Client");
  }

  ngOnInit(): void {
    console.log("ClientComponent::ngOnInit()");
    this.client_id = this.route.snapshot.paramMap.get("client_id") || '';
    this.action = this.route.snapshot.paramMap.get("action") || "new";

    this.country_service.fetch().subscribe((countries: Array<Country>) => {
      this.countries = countries;
    });

    if (this.action != "new") {
      this.client_service.get(this.client_id).subscribe((r: ApiResponse) => {
        if (r.success) {
          this.client = (r.data as { client: Client }).client;
          this.set_client(this.client);
        } else {
          console.error("ClientComponent::ngOnInit()", r);
        }
      });
    }
  }

  get client_name() {
    return this.clientForm.get("name");
  }

  get client_active() {
    return this.clientForm.get("active");
  }

  get client_description() {
    return this.clientForm.get("description");
  }

  get client_address() {
    return this.clientForm.get("address");
  }

  get client_country() {
    return this.clientForm.get("country");
  }

  get client_url() {
    return this.clientForm.get("url");
  }

  // compareCountry(o1: any, o2: any):boolean {
  //   console.log(o1, o2);
  //   return o1 && o2 && o1 == o2;
  // }

  set_client(client: Client) {
    this.clientForm.get("name")?.setValue(client.name);
    this.clientForm.get("active")?.setValue(client.active);
    this.clientForm.get("description")?.setValue(client.description);
    this.clientForm.get("address")?.setValue(client.address);
    this.clientForm.get("country")?.setValue(client.country_id);
    this.clientForm.get("address")?.setValue(client.address);
    this.clientForm.get("url")?.setValue(client.url);
    
  }

  submit() {
    console.log("ClientComponent::submit()");
    if (this.clientForm.valid) {
      this.submitting = true;
      switch (this.action) {
        case "edit": {
          this.client_service.update(
            this.client_id,
            this.client_name?.value || '',
            this.client_description?.value || '',
            this.client_address?.value || '',
            this.client_country?.value || 0,
            this.client_url?.value || ''
          ).subscribe((r: ApiResponse) => {
            if (!r.success) {
              console.error("error: ", r);
            }
            this.submitting = false;
          });
          break;
        }
        default: {
          this.client_service.add(
            this.client_name?.value || '',
            this.client_description?.value || '',
            this.client_address?.value || '',
            this.client_country?.value || 0,
            this.client_url?.value || ''
          ).subscribe((r: ApiResponse) => {
            if (!r.success) {
              console.error("error: ", r);
            }
            this.submitting = false;
          });
          break;
        }
      }
      
    }
  }
}
