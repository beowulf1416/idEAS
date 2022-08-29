import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';

import { CompleteComponent } from './components/complete/complete.component';
import { RegisterComponent } from './components/register/register.component';

const routes: Routes = [
  {
    path: "register",
    component: RegisterComponent
  },
  {
    path: "register/complete/:token",
    component: CompleteComponent
  }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
