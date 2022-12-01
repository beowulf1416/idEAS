import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { PermissionGuard } from 'src/app/guards/permission.guard';
import { AdminComponent } from './components/admin/admin.component';
import { ClientListComponent } from './components/client-list/client-list.component';
import { ClientToolbarComponent } from './components/client-toolbar/client-toolbar.component';
import { ClientComponent } from './components/client/client.component';
import { DashboardComponent } from './components/dashboard/dashboard.component';
import { MemberInviteComponent } from './components/member-invite/member-invite.component';
import { MemberListComponent } from './components/member-list/member-list.component';
import { OrganizationListComponent } from './components/organization-list/organization-list.component';
import { RoleListComponent } from './components/role-list/role-list.component';

const routes: Routes = [
  {
    path: "client",
    component: AdminComponent,
    children: [
      {
        path: "list",
        component: ClientListComponent
      },
      {
        path: "new",
        component: ClientComponent
      },
      
      {
        path: "view/:client_id",
        component: ClientComponent,
        data: {
          action: "view"
        }
      },
      {
        path: "edit/:client_id",
        component: ClientComponent,
        data: {
          action: "edit"
        }
      },
      {
        path: "members/:client_id",
        component: MemberListComponent
      },
      {
        path: "organizations/:client_id",
        component: OrganizationListComponent
      },
      {
        path: "invite/:client_id",
        component: MemberInviteComponent
      },
      {
        path: "roles/:client_id",
        component: RoleListComponent
      }
    ]
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class ClientRoutingModule { }
