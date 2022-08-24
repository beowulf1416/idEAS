import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ReactiveFormsModule } from '@angular/forms';

import { ProfileRoutingModule } from './profile-routing.module';
import { PasswordComponent } from './components/password/password.component';
import { ProfileComponent } from './components/profile/profile.component';



@NgModule({
  declarations: [
    PasswordComponent,
    ProfileComponent
  ],
  imports: [
    CommonModule,
    ReactiveFormsModule,
    ProfileRoutingModule
  ]
})
export class ProfileModule { }
