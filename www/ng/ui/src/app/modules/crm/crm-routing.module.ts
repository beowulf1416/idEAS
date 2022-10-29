import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { DashboardComponent } from './components/dashboard/dashboard.component';
import { PeopleComponent } from './components/people/people.component';
import { PersonComponent } from './components/person/person.component';

const routes: Routes = [
  {
    path: "people/new",
    component: PersonComponent
  },
  {
    path: "person/:people_id",
    component: PersonComponent
  },
  {
    path: "person/:people_id/edit",
    component: PersonComponent
  },
  {
    path: "people",
    component: PeopleComponent
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
export class CrmRoutingModule { }
