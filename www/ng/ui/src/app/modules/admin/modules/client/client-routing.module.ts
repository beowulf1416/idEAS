import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { ClientListComponent } from './components/client-list/client-list.component';
import { ClientComponent } from './components/client/client.component';
import { DashboardComponent } from './components/dashboard/dashboard.component';

const routes: Routes = [
  {
    path: "new",
    component: ClientComponent
  },
  {
    path: "client/:client_id",
    component: ClientComponent
  },
  {
    path: "client/:client_id/:action",
    component: ClientComponent
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
