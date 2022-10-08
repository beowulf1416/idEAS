import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { ReactiveFormsModule } from '@angular/forms';
import { MaterialModule } from 'src/material/material.module';
import { HttpClientModule } from '@angular/common/http';

import { AppRoutingModule } from './app-routing.module';
import { httpInterceptorProviders } from './interceptors';

import { AppComponent } from './app.component';
import { WorkspaceComponent } from './components/workspace/workspace.component';
import { HomeComponent } from './components/home/home.component';
import { ClientSelectComponent } from './components/client-select/client-select.component';

@NgModule({
  declarations: [
    AppComponent,
    WorkspaceComponent,
    HomeComponent,
    ClientSelectComponent
  ],
  imports: [
    BrowserModule,
    BrowserAnimationsModule,
    ReactiveFormsModule,
    MaterialModule,
    HttpClientModule,
    AppRoutingModule
  ],
  providers: [
    httpInterceptorProviders
  ],
  bootstrap: [AppComponent]
})
export class AppModule { }
