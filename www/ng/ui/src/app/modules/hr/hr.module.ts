import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { HrRoutingModule } from './hr-routing.module';
import { HomeComponent } from './components/home/home.component';
import { DashboardComponent } from './components/dashboard/dashboard.component';
import { ReactiveFormsModule } from '@angular/forms';
import { MaterialModule } from 'src/material/material.module';
import { EmployeeComponent } from './components/employee/employee.component';
import { EmployeeListComponent } from './components/employee-list/employee-list.component';
import { LeaveComponent } from './components/leave/leave.component';
import { TimesheetComponent } from './components/timesheet/timesheet.component';


@NgModule({
  declarations: [
    HomeComponent,
    DashboardComponent,
    EmployeeComponent,
    EmployeeListComponent,
    LeaveComponent,
    TimesheetComponent
  ],
  imports: [
    CommonModule,
    ReactiveFormsModule,
    MaterialModule,
    HrRoutingModule
  ]
})
export class HrModule { }
