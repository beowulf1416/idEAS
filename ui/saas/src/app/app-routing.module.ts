import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { ForbiddenComponent } from './components/forbidden/forbidden.component';
import { AuthGuard } from './guards/auth.guard';
import { ProfileModule } from './modules/auth/profile/profile.module';
import { TenantAdminModule } from './modules/auth/tenant-admin/tenant-admin.module';

const routes: Routes = [
  {
    path: 'user/signin',
    loadChildren: () => import('./modules/anonymous/signin/signin.module').then(m => m.SigninModule)
  },
  {
    path: 'user/signup',
    loadChildren: () => import('./modules/anonymous/signup/signup.module').then(m => m.SignupModule)
  },
  {
    path: 'dashboard',
    canActivate: [AuthGuard],
    data: {
      permission: 'dashboard.view'
    },
    loadChildren: () => import('./modules/auth/dashboard/dashboard.module').then(m => m.DashboardModule)
  },
  {
    path: 'tenant',
    canActivate: [AuthGuard],
    loadChildren: () => import('./modules/auth/tenants/tenants.module').then(m => m.TenantsModule)
  },
  {
    path: 'user/profile',
    data: {
      permission: "user.profile"
    },
    loadChildren: () => import('./modules/auth/profile/profile.module').then(m => ProfileModule)
  },
  {
    path: 'tenant/admin',
    loadChildren: () => import('./modules/auth/tenant-admin/tenant-admin.module').then(m => TenantAdminModule)
  },
  {
    path: 'error/forbidden',
    component: ForbiddenComponent
  }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
