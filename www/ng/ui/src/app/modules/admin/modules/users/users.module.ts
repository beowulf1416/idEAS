import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { UsersRoutingModule } from './users-routing.module';
import { AdminComponent } from './components/admin/admin.component';
import { MaterialModule } from 'src/material/material.module';
import { UserComponent } from './components/user/user.component';


@NgModule({
  declarations: [
    AdminComponent,
    UserComponent
  ],
  imports: [
    CommonModule,
    MaterialModule,
    UsersRoutingModule
  ]
})
export class UsersModule { }
