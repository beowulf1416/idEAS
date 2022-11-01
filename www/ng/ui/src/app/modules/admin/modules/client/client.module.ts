import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ReactiveFormsModule } from '@angular/forms';

import { ClientRoutingModule } from './client-routing.module';
import { DashboardComponent } from './components/dashboard/dashboard.component';
import { ClientComponent } from './components/client/client.component';
import { MaterialModule } from 'src/material/material.module';
import { ClientListComponent } from './components/client-list/client-list.component';
import { ClientToolbarComponent } from './components/client-toolbar/client-toolbar.component';


@NgModule({
  declarations: [
    DashboardComponent,
    ClientComponent,
    ClientListComponent,
    ClientToolbarComponent
  ],
  imports: [
    CommonModule,
    ReactiveFormsModule,
    MaterialModule,
    ClientRoutingModule
  ]
})
export class ClientModule { }
