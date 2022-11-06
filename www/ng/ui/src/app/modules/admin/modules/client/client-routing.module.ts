import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { PermissionGuard } from 'src/app/guards/permission.guard';
import { ClientListComponent } from './components/client-list/client-list.component';
import { ClientComponent } from './components/client/client.component';
import { DashboardComponent } from './components/dashboard/dashboard.component';

const routes: Routes = [
  {
    path: "new",
    component: ClientComponent,
    canActivate: [ PermissionGuard ],
    data: {
      permission: "client.add"
    }
  },
  {
    path: "client/:client_id",
    component: ClientComponent
  },
  {
    path: "client/:client_id/:action",
    component: ClientComponent,
    data: {
      permission: "client.add"
    }
  },
  {
    path: "list",
    component: ClientListComponent
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
