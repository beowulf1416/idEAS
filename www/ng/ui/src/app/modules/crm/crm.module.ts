import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { CrmRoutingModule } from './crm-routing.module';
import { PersonComponent } from './components/person/person.component';
import { PeopleComponent } from './components/people/people.component';


@NgModule({
  declarations: [
    PersonComponent,
    PeopleComponent
  ],
  imports: [
    CommonModule,
    CrmRoutingModule
  ]
})
export class CrmModule { }
