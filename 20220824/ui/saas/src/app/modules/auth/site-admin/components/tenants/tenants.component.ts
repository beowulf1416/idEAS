import { Component, OnInit } from '@angular/core';
import { TitleService } from 'src/app/services/title.service';

import { Tenant } from '../../classes/tenant';
import { TenantsService } from '../../services/tenants.service';

@Component({
  selector: 'app-tenants',
  templateUrl: './tenants.component.html',
  styleUrls: ['./tenants.component.css']
})
export class TenantsComponent implements OnInit {

  tenants: Array<Tenant> = new Array<Tenant>();

  constructor(
    private title: TitleService,
    private tenants_service: TenantsService
  ) { 
    this.title.set_title("Tenants");
  }

  ngOnInit(): void {
    this.tenants_service.get_tenants('%', 10, 0).subscribe((tenants: Array<Tenant>) => {
      this.tenants = tenants;
    });
  }

}
