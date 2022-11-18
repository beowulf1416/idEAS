import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AccountChartComponent } from './account-chart.component';

describe('AccountChartComponent', () => {
  let component: AccountChartComponent;
  let fixture: ComponentFixture<AccountChartComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ AccountChartComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AccountChartComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
