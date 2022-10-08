import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { RolesRoutingModule } from './roles-routing.module';
import { RoleComponent } from './components/role/role.component';
import { RoleListComponent } from './components/role-list/role-list.component';


@NgModule({
  declarations: [
    RoleComponent,
    RoleListComponent
  ],
  imports: [
    CommonModule,
    RolesRoutingModule
  ]
})
export class RolesModule { }
