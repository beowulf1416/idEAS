import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { TitleService } from 'src/app/services/title.service';
import { TenantService } from '../../services/tenant.service';

interface Tenant {
  id: string,
  name: string
}

@Component({
  selector: 'app-tenant-select',
  templateUrl: './tenant-select.component.html',
  styleUrls: ['./tenant-select.component.css']
})
export class TenantSelectComponent implements OnInit {

  tenants = [{ id: '', name: '' }];
  tenantsForm = new FormGroup({

  });

  constructor(
    private title: TitleService,
    private service: TenantService
  ) { 
    this.title.set_title("Select Tenant");
  }

  ngOnInit(): void {
    console.log("TenantSelectComponent::ngOnInit()");
    this.service.get_tenants().subscribe(r => {
      if (r.status == "success") {
        // this.tenants = r.data?.tenants;
        this.rebuildForm(r.data?.tenants);
      } else {
        console.error(r.message);
      }
    });
  }

  rebuildForm(tenants: [Tenant]) {
    this.tenants = tenants;
    let group: any = {};

    tenants.forEach(t => {
      group["tenant_" + t.id] = new FormControl();
    });

    this.tenantsForm = new FormGroup(group);
  }

  select() {
    console.log("TenantSelectComponent::select()");

  }
}
