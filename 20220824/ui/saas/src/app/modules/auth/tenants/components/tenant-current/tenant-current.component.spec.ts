import { ComponentFixture, TestBed } from '@angular/core/testing';

import { TenantCurrentComponent } from './tenant-current.component';

describe('TenantCurrentComponent', () => {
  let component: TenantCurrentComponent;
  let fixture: ComponentFixture<TenantCurrentComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ TenantCurrentComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(TenantCurrentComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
