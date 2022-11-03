import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { ProfileRoutingModule } from './profile-routing.module';
import { GeneralComponent } from './components/general/general.component';
import { ReactiveFormsModule } from '@angular/forms';
import { MaterialModule } from 'src/material/material.module';
import { PasswordComponent } from './components/password/password.component';


@NgModule({
  declarations: [
    GeneralComponent,
    PasswordComponent
  ],
  imports: [
    CommonModule,
    ReactiveFormsModule,
    MaterialModule,
    ProfileRoutingModule
  ]
})
export class ProfileModule { }
