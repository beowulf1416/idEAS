import { NgModule } from '@angular/core';
import { SignupRoutingModule } from './signup-routing.module';
import { SignupComponent } from './components/signup/signup.component';



@NgModule({
  declarations: [
    SignupComponent
  ],
  imports: [
    SignupRoutingModule
  ]
})
export class SignupModule { }
