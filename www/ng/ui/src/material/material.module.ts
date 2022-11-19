import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatCheckboxModule } from '@angular/material/checkbox';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatListModule } from '@angular/material/list';
import { MatMenuModule } from '@angular/material/menu';
import { MatSlideToggleModule } from '@angular/material/slide-toggle';
import { MatPaginatorModule } from '@angular/material/paginator';
import { MatSelectModule } from '@angular/material/select';
import { MatSnackBarModule } from '@angular/material/snack-bar';

// import {MatLegacyFormFieldModule as MatFormFieldModule} from '@angular/material/legacy-form-field';
// import {MatLegacyInputModule as MatInputModule} from '@angular/material/legacy-input';
// import {MatLegacyCheckboxModule as MatCheckboxModule} from '@angular/material/legacy-checkbox';
// import {MatLegacyButtonModule as MatButtonModule} from '@angular/material/legacy-button';
// import {MatIconModule} from '@angular/material/icon';
import {MatToolbarModule} from '@angular/material/toolbar';
import {MatSidenavModule} from '@angular/material/sidenav';
// import {MatLegacyListModule as MatListModule} from '@angular/material/legacy-list';
import {MatExpansionModule} from '@angular/material/expansion';
// import {MatLegacyMenuModule as MatMenuModule} from '@angular/material/legacy-menu';
import {MatBadgeModule} from '@angular/material/badge';
// import {MatLegacySlideToggleModule as MatSlideToggleModule} from '@angular/material/legacy-slide-toggle';
// import {MatLegacyPaginatorModule as MatPaginatorModule} from '@angular/material/legacy-paginator';
import {MatDividerModule} from '@angular/material/divider';
// import {MatLegacySelectModule as MatSelectModule} from '@angular/material/legacy-select';
// import {MatLegacySnackBarModule as MatSnackBarModule} from '@angular/material/legacy-snack-bar';
import { MatTableModule } from '@angular/material/table';

@NgModule({
  declarations: [],
  imports: [
    CommonModule
  ],
  exports: [
    MatFormFieldModule,
    MatInputModule,
    MatCheckboxModule,
    MatButtonModule,
    MatIconModule,
    MatToolbarModule,
    MatSidenavModule,
    MatListModule,
    MatExpansionModule,
    MatMenuModule,
    MatBadgeModule,
    MatSlideToggleModule,
    MatPaginatorModule,
    MatDividerModule,
    MatSelectModule,
    MatSnackBarModule,
    MatTableModule
  ]
})
export class MaterialModule { }
