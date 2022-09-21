import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { RegisterRoutingModule } from './register-routing.module';
import { StartComponent } from './components/start/start.component';
import { CompleteComponent } from './components/complete/complete.component';


@NgModule({
  declarations: [
    StartComponent,
    CompleteComponent
  ],
  imports: [
    CommonModule,
    RegisterRoutingModule
  ]
})
export class RegisterModule { }
