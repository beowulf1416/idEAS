import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';

import { AuthGuard } from 'src/app/guards/auth.guard';
import { PasswordComponent } from './components/password/password.component';
import { ProfileComponent } from './components/profile/profile.component';


const routes: Routes = [
  {
    path: '',
    canActivate: [AuthGuard],
    component: ProfileComponent
  },
  {
    path: 'password',
    canActivate: [AuthGuard],
    component: PasswordComponent
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class ProfileRoutingModule { }
