import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { AdminComponent } from './components/admin/admin.component';
import { UserListComponent } from './components/user-list/user-list.component';
import { UserComponent } from './components/user/user.component';

const routes: Routes = [
  {
    path: "",
    component: AdminComponent,
    children: [
      {
        path: 'new',
        component: UserComponent
      },
      {
        path: 'edit/:user_id',
        component: UserComponent
      },
      {
        path: 'view/:user_id',
        component: UserComponent,
        data: {
          readonly: true
        }
      },
      {
        path: 'list',
        component: UserListComponent
      },
      {
        path: 'invite/:client_id',
        component: UserComponent,
        data: {
          invite: true
        }
      }
    ]
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class UsersRoutingModule { }
