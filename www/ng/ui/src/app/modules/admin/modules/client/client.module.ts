import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ReactiveFormsModule } from '@angular/forms';

import { ClientRoutingModule } from './client-routing.module';
import { DashboardComponent } from './components/dashboard/dashboard.component';
import { ClientComponent } from './components/client/client.component';
import { MaterialModule } from 'src/material/material.module';
import { ClientListComponent } from './components/client-list/client-list.component';
import { ClientToolbarComponent } from './components/client-toolbar/client-toolbar.component';
import { MemberListComponent } from './components/member-list/member-list.component';
import { ClientDashboardComponent } from './components/client-dashboard/client-dashboard.component';
import { AdminComponent } from './components/admin/admin.component';
import { MemberItemComponent } from './components/member-item/member-item.component';
import { OrganizationListComponent } from './components/organization-list/organization-list.component';
import { OrganizationComponent } from './components/organization/organization.component';
import { MemberInviteComponent } from './components/member-invite/member-invite.component';
import { RolesComponent } from './components/roles/roles.component';
import { RoleListComponent } from './components/role-list/role-list.component';
import { RoleComponent } from './components/role/role.component';


@NgModule({
  declarations: [
    DashboardComponent,
    ClientComponent,
    ClientListComponent,
    ClientToolbarComponent,
    MemberListComponent,
    ClientDashboardComponent,
    AdminComponent,
    MemberItemComponent,
    OrganizationListComponent,
    OrganizationComponent,
    MemberInviteComponent,
    RolesComponent,
    RoleListComponent,
    RoleComponent
  ],
  imports: [
    CommonModule,
    ReactiveFormsModule,
    MaterialModule,
    ClientRoutingModule
  ]
})
export class ClientModule { }
