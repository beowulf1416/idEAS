import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { AuthGuard } from 'src/app/guards/auth.guard';

import { TenantComponent } from './components/tenant/tenant.component';
import { TenantsDashboardComponent } from './components/tenants-dashboard/tenants-dashboard.component';
import { TenantsListComponent } from './components/tenants-list/tenants-list.component';

const routes: Routes = [
  {
    path: '',
    canActivate: [AuthGuard],
    data: {
      permission: 'tenants.admin'
    },
    component: TenantsDashboardComponent
  },
  {
    path: 'dashboard',
    canActivate: [AuthGuard],
    data: {
      permission: 'tenants.admin'
    },
    component: TenantsDashboardComponent
  },
  {
    path: 'list',
    canActivate: [AuthGuard],
    data: {
      permission: 'tenants.admin.list'
    },
    component: TenantsListComponent
  },
  {
    path: 'view',
    canActivate: [AuthGuard],
    data: {
      permission: 'tenants.admin.view'
    },
    component: TenantComponent
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class TenantsRoutingModule { }