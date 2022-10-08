import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { ClientSelectComponent } from './components/client-select/client-select.component';

import { HomeComponent } from './components/home/home.component';
// import { WorkspaceComponent } from './components/workspace/workspace.component';

const routes: Routes = [
  {
    path: "auth",
    loadChildren: () => import('./modules/auth/auth.module').then(m => m.AuthModule)
  },
  {
    path: "register",
    loadChildren: () => import('./modules/register/register.module').then(m => m.RegisterModule)
  },
  {
    path: "dashboard",
    loadChildren: () => import('./modules/dashboard/dashboard.module').then(m => m.DashboardModule)
  },
  {
    path: "admin",
    loadChildren: () => import('./modules/admin/admin.module').then(m => m.AdminModule)
  },
  {
    path: "accounting",
    loadChildren: () => import('./modules/accounting/accounting.module').then(m => m.AccountingModule)
  },
  {
    path: "inventory",
    loadChildren: () => import('./modules/inventory/inventory.module').then(m => m.InventoryModule)
  },
  {
    path: "crm",
    loadChildren: () => import('./modules/crm/crm.module').then(m => m.CrmModule)
  },
  {
    path: "client/join",
    component: ClientSelectComponent
  },
  {
    path: "",
    component: HomeComponent
  }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
