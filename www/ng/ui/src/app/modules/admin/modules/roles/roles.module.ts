import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MaterialModule } from 'src/material/material.module';

import { RolesRoutingModule } from './roles-routing.module';
import { RoleComponent } from './components/role/role.component';
import { RoleListComponent } from './components/role-list/role-list.component';
import { ReactiveFormsModule } from '@angular/forms';
import { RolePermissionsComponent } from './components/role-permissions/role-permissions.component';


@NgModule({
  declarations: [
    RoleComponent,
    RoleListComponent,
    RolePermissionsComponent
  ],
  imports: [
    CommonModule,
    MaterialModule,
    ReactiveFormsModule,
    RolesRoutingModule
  ]
})
export class RolesModule { }
