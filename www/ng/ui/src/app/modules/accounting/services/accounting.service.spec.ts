import { TestBed } from '@angular/core/testing';

import { AccountingService } from './accounting.service';

describe('AccountingService', () => {
  let service: AccountingService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(AccountingService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
