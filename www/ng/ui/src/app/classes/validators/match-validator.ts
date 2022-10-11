import { AbstractControl, ValidationErrors, ValidatorFn } from '@angular/forms';

export function matchValidator(control1: string, control2: string): ValidatorFn {
    return (control: AbstractControl): ValidationErrors | null => {
      const value1 = control.get(control1)?.value;
      const value2 = control.get(control2)?.value;

      return value1 == value2 ? null : { identical: true };
    }
}