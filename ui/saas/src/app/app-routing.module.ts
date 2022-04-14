import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';

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
    loadChildren: () => import('./modules/auth/dashboard/dashboard.module').then(m => m.DashboardModule)
  }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
