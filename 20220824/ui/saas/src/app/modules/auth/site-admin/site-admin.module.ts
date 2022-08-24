import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { SiteAdminRoutingModule } from './site-admin-routing.module';
import { TenantsComponent } from './components/tenants/tenants.component';
import { DashboardComponent } from './components/dashboard/dashboard.component';


@NgModule({
  declarations: [
    TenantsComponent,
    DashboardComponent
  ],
  imports: [
    CommonModule,
    SiteAdminRoutingModule
  ]
})
export class SiteAdminModule { }
