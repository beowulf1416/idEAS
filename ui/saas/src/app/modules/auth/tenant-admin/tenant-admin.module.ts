import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { TenantAdminRoutingModule } from './tenant-admin-routing.module';
import { UsersComponent } from './components/users/users.component';



@NgModule({
  declarations: [
    UsersComponent
  ],
  imports: [
    CommonModule,
    TenantAdminRoutingModule
  ]
})
export class TenantAdminModule { }
