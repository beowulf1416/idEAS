import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MaterialModule } from 'src/material/material.module';

import { RolesRoutingModule } from './roles-routing.module';
import { RoleComponent } from './components/role/role.component';
import { RoleListComponent } from './components/role-list/role-list.component';
import { ReactiveFormsModule } from '@angular/forms';


@NgModule({
  declarations: [
    RoleComponent,
    RoleListComponent
  ],
  imports: [
    CommonModule,
    MaterialModule,
    ReactiveFormsModule,
    RolesRoutingModule
  ]
})
export class RolesModule { }
