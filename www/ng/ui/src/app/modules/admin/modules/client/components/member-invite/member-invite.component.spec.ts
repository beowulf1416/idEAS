import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MemberInviteComponent } from './member-invite.component';

describe('MemberInviteComponent', () => {
  let component: MemberInviteComponent;
  let fixture: ComponentFixture<MemberInviteComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ MemberInviteComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(MemberInviteComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
