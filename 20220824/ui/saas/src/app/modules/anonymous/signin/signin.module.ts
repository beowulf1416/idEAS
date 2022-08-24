import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ReactiveFormsModule } from '@angular/forms';

import { SigninComponent } from './components/signin/signin.component';
import { SigninRoutingModule } from './signin-routing.module';



@NgModule({
  declarations: [
    SigninComponent
  ],
  imports: [
    CommonModule,
    ReactiveFormsModule,
    SigninRoutingModule
  ]
})
export class SigninModule { }
