import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { CompleteComponent } from './components/complete/complete.component';
import { StartComponent } from './components/start/start.component';

const routes: Routes = [
  {
    path: "start",
    component: StartComponent
  },
  {
    path: "complete/:token",
    component: CompleteComponent
  },
  {
    path: "",
    component: StartComponent
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class RegisterRoutingModule { }
