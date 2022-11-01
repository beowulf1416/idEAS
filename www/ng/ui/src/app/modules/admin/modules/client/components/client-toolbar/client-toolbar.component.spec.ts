import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ClientToolbarComponent } from './client-toolbar.component';

describe('ClientToolbarComponent', () => {
  let component: ClientToolbarComponent;
  let fixture: ComponentFixture<ClientToolbarComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ClientToolbarComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ClientToolbarComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
