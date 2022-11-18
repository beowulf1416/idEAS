import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { AccountChartComponent } from './components/account-chart/account-chart.component';
import { AccountComponent } from './components/account/account.component';
import { DashboardComponent } from './components/dashboard/dashboard.component';
import { HomeComponent } from './components/home/home.component';

const routes: Routes = [
  {
    path: "",
    component: HomeComponent,
    children: [
      {
        path: "",
        component: DashboardComponent
      },
      {
        path: "chart",
        component: AccountChartComponent
      },
      {
        path: "account",
        component: AccountComponent
      }
    ]
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class AccountingRoutingModule { }
