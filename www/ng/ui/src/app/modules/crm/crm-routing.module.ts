import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { PeopleComponent } from './components/people/people.component';
import { PersonComponent } from './components/person/person.component';

const routes: Routes = [
  {
    path: "person",
    component: PersonComponent
  },
  {
    path: "people",
    component: PeopleComponent
  },
  {
    path: "",
    component: PeopleComponent
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class CrmRoutingModule { }
