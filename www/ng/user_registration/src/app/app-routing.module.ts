import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';

import { CompleteComponent } from './components/complete/complete.component';
import { RegisterComponent } from './components/register/register.component';

const routes: Routes = [
  {
    path: "complete/:token",
    component: CompleteComponent
  },
  {
    path: "register",
    component: RegisterComponent
  }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
