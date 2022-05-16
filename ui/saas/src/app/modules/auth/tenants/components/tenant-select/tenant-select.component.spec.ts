import { ComponentFixture, TestBed } from '@angular/core/testing';

import { TenantSelectComponent } from './tenant-select.component';

describe('TenantSelectComponent', () => {
  let component: TenantSelectComponent;
  let fixture: ComponentFixture<TenantSelectComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ TenantSelectComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(TenantSelectComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
