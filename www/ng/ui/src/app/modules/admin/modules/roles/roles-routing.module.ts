import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { RoleListComponent } from './components/role-list/role-list.component';
import { RoleComponent } from './components/role/role.component';

const routes: Routes = [
  {
    path: "new",
    component: RoleComponent
  },
  {
    path: "role/:role_id",
    component: RoleComponent
  },
  {
    path: "role/:role_id/:action",
    component: RoleComponent
  },
  {
    path: "",
    component: RoleListComponent
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class RolesRoutingModule { }
