import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { GeneralComponent } from './components/general/general.component';
import { PasswordComponent } from './components/password/password.component';

const routes: Routes = [
  {
    path: "",
    component: GeneralComponent
  },
  {
    path: "password",
    component: PasswordComponent
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class ProfileRoutingModule { }
