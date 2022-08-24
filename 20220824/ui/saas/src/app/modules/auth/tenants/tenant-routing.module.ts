import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';

import { AuthGuard } from 'src/app/guards/auth.guard';
import { TenantCurrentComponent } from './components/tenant-current/tenant-current.component';
import { TenantSelectComponent } from './components/tenant-select/tenant-select.component';

const routes: Routes = [
  {
    path: '',
    canActivate: [AuthGuard],
    component: TenantCurrentComponent
  },
  {
    path: 'current',
    canActivate: [AuthGuard],
    component: TenantCurrentComponent
  },
  {
    path: 'select',
    canActivate: [AuthGuard],
    component: TenantSelectComponent
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class TenantRoutingModule { }
