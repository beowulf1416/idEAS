import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { Router } from '@angular/router';
import { TitleService } from 'src/app/services/title.service';
import { environment } from 'src/environments/environment';
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
    tenant: new FormControl('')
  });

  constructor(
    private title: TitleService,
    private service: TenantService,
    private router: Router
  ) { 
    this.title.set_title("Select Tenant");
  }

  ngOnInit(): void {
    console.log("TenantSelectComponent::ngOnInit()");
    this.service.get_tenants().subscribe(r => {
      if (r.status == "success") {
        // this.tenants = r.data?.tenants;
        // this.rebuildForm(r.data?.tenants);
        this.tenants = r.data?.tenants;
      } else {
        console.error(r.message);
      }
    });
  }

  // rebuildForm(tenants: [Tenant]) {
  //   this.tenants = tenants;
  //   let group: any = {};

  //   tenants.forEach(t => {
  //     group["tenant_" + t.id] = new FormControl();
  //   });

  //   this.tenantsForm = new FormGroup(group);
  // }

  get tenant() {
    return this.tenantsForm.get("tenant");
  }

  select() {
    console.log("TenantSelectComponent::select()");
    // console.log(this.tenantsForm);
    this.service.select_tenant(
      this.tenantsForm.get("tenant")?.value
    ).subscribe(r => {
      if (r.body?.status == "success") {
        console.log(r.body);

        console.log("redirecting to dashboard");
        this.router.navigate([environment.path_dashboard]);
      } else {
        console.error(r.body);
      }
    });
  }
}
