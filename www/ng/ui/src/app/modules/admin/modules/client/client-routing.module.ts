import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { PermissionGuard } from 'src/app/guards/permission.guard';
import { AdminComponent } from './components/admin/admin.component';
import { ClientListComponent } from './components/client-list/client-list.component';
import { ClientToolbarComponent } from './components/client-toolbar/client-toolbar.component';
import { ClientComponent } from './components/client/client.component';
import { DashboardComponent } from './components/dashboard/dashboard.component';
import { MemberListComponent } from './components/member-list/member-list.component';

const routes: Routes = [
  // {
  //   path: "new",
  //   component: ClientComponent,
  //   canActivate: [ PermissionGuard ],
  //   data: {
  //     permission: "client.add"
  //   }
  // },
  // {
  //   path: "client/:client_id",
  //   component: ClientComponent
  // },
  // {
  //   path: "client/:client_id/:action",
  //   component: ClientComponent
  // },
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
      }
    ]
  },
  {
    path: "",
    component: DashboardComponent
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class ClientRoutingModule { }
