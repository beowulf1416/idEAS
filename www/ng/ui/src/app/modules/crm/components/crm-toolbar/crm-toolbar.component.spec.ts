import { ComponentFixture, TestBed } from '@angular/core/testing';

import { CrmToolbarComponent } from './crm-toolbar.component';

describe('CrmToolbarComponent', () => {
  let component: CrmToolbarComponent;
  let fixture: ComponentFixture<CrmToolbarComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ CrmToolbarComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(CrmToolbarComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
