import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ReactiveFormsModule } from '@angular/forms';

import { TenantRoutingModule } from './tenant-routing.module';
import { TenantSelectComponent } from './components/tenant-select/tenant-select.component';
import { TenantCurrentComponent } from './components/tenant-current/tenant-current.component';



@NgModule({
  declarations: [
    TenantSelectComponent,
    TenantCurrentComponent
  ],
  imports: [
    CommonModule,
    ReactiveFormsModule,
    TenantRoutingModule
  ]
})
export class TenantsModule { }
