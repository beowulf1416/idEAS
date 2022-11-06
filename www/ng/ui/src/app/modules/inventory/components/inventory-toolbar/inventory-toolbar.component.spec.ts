import { ComponentFixture, TestBed } from '@angular/core/testing';

import { InventoryToolbarComponent } from './inventory-toolbar.component';

describe('InventoryToolbarComponent', () => {
  let component: InventoryToolbarComponent;
  let fixture: ComponentFixture<InventoryToolbarComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ InventoryToolbarComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(InventoryToolbarComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
