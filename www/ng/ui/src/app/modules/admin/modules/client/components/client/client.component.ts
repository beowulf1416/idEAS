import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { timeStamp } from 'console';
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

  countries: Array<Country> = [];

  clientForm = new FormGroup({
    name: new FormControl('', []),
    description: new FormControl('', []),
    address: new FormControl('', []),
    country: new FormControl('', []),
    url: new FormControl('', [])
  });

  constructor(
    private title: TitleService,
    private client_service: ClientService,
    private country_service: CountryService
  ) {
    this.title.set_title("Client");
  }

  ngOnInit(): void {
    this.country_service.fetch().subscribe(countries => {
      this.countries = countries;
    });
  }

  get client_name() {
    return this.clientForm.get("name");
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

  submit() {
    console.log("ClientComponent::submit()");
    if (this.clientForm.valid) {
      this.submitting = true;
      this.client_service.add(
        this.client_name?.value || '',
        this.client_description?.value || '',
        this.client_address?.value || '',
        parseInt(this.client_country?.value || ''),
        this.client_url?.value || ''
      ).subscribe(r => {
        if (!r.success) {
          console.error("error: ", r);
        }
        this.submitting = false;
      });
    }
  }
}
