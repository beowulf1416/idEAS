import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ReactiveFormsModule } from '@angular/forms';
import { MaterialModule } from 'src/material/material.module';

import { CrmRoutingModule } from './crm-routing.module';
import { PersonComponent } from './components/person/person.component';
import { PeopleComponent } from './components/people/people.component';
import { DashboardComponent } from './components/dashboard/dashboard.component';
import { CrmToolbarComponent } from './components/crm-toolbar/crm-toolbar.component';


@NgModule({
  declarations: [
    PersonComponent,
    PeopleComponent,
    DashboardComponent,
    CrmToolbarComponent
  ],
  imports: [
    CommonModule,
    ReactiveFormsModule,
    MaterialModule,
    CrmRoutingModule
  ]
})
export class CrmModule { }
