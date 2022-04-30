import { NgModule } from '@angular/core';

import { AuthGuard } from 'src/app/guards/auth.guard';
import { PasswordComponent } from './components/password/password.component';


const routes: Routes = [
  {
    path: '',
    canActivate: [AuthGuard],
    component: PasswordComponent
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class ProfileRoutingModule { }
