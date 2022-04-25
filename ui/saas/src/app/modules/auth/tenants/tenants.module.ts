import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { TenantComponent } from './components/tenant/tenant.component';
import { TenantsRoutingModule } from './tenants-routing.module';



@NgModule({
  declarations: [
    TenantComponent
  ],
  imports: [
    CommonModule,
    TenantsRoutingModule
  ]
})
export class TenantsModule { }
