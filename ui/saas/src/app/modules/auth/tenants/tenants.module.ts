import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { TenantComponent } from './components/tenant/tenant.component';
import { TenantsRoutingModule } from './tenants-routing.module';
import { TenantsListComponent } from './components/tenants-list/tenants-list.component';
import { TenantsDashboardComponent } from './components/tenants-dashboard/tenants-dashboard.component';



@NgModule({
  declarations: [
    TenantComponent,
    TenantsListComponent,
    TenantsDashboardComponent
  ],
  imports: [
    CommonModule,
    TenantsRoutingModule
  ]
})
export class TenantsModule { }
