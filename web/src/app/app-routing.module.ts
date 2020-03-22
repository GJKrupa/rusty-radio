import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import {MainComponent} from "./main/main.component";
import {AlarmComponent} from "./alarm/alarm.component";
import {RadioComponent} from "./radio/radio.component";
import {CreateComponent} from "./alarm/create/create.component";


const routes: Routes = [
  { path: 'main', component: MainComponent },
  { path: 'radio', component: RadioComponent },
  { path: 'alarm', component: AlarmComponent },
  { path: 'alarm/create', component: CreateComponent },
  { path: '',
    redirectTo: '/main',
    pathMatch: 'full'
  },
];

@NgModule({
  imports: [RouterModule.forRoot(routes, {useHash: true})],
  exports: [RouterModule]
})
export class AppRoutingModule { }
